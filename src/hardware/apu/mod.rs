use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use blip_buf::BlipBuf;

use crate::hardware::{
    apu::{
        pulse_channel::{PulseChannel, PulseChannelType},
        triangle_channel::TriangleChannel,
    },
    bit_ops::BitOps,
    constants::{
        apu::{
            BLIP_BUFFER_SIZE, BLIP_FRAME_SIZE, BLIP_SCALE, frame_counter_register, status_register,
        },
        clock_rates::{CPU_CLOCK, SAMPLE_RATE},
    },
    cpu::Cpu,
};

pub mod envelope;
pub mod length_counter;
pub mod pulse_channel;
pub mod sweep;
pub mod triangle_channel;

#[derive(Default, Clone, Copy, Debug)]
pub struct ApuTick {
    pub is_apu_cycle: bool,
    pub is_quarter_frame: bool,
    pub is_half_frame: bool,
}

/// https://www.nesdev.org/wiki/APU
pub struct Apu {
    pulse1: PulseChannel,
    pulse2: PulseChannel,
    triangle: TriangleChannel,

    sequencer_mode_flag: bool,
    interrupt_inhibit_flag: bool,
    frame_interrupt_flag: bool,

    cpu_total_cycles: usize,
    apu_total_cycles: usize,
    new_mode_flag: bool,
    new_mode_flag_cycle: usize,

    blip: BlipBuf,
    blip_clock: u32,
    prev_blip_output: i32,
    sample_queue: VecDeque<f32>,
}

impl Apu {
    pub fn new() -> Self {
        let mut blip = BlipBuf::new(BLIP_BUFFER_SIZE);
        blip.set_rates(CPU_CLOCK as f64, SAMPLE_RATE as f64);
        Self {
            pulse1: PulseChannel::new(PulseChannelType::Pulse1),
            pulse2: PulseChannel::new(PulseChannelType::Pulse2),
            triangle: TriangleChannel::default(),
            sequencer_mode_flag: false,
            interrupt_inhibit_flag: false,
            frame_interrupt_flag: false,
            cpu_total_cycles: 0,
            apu_total_cycles: 0,
            new_mode_flag: false,
            new_mode_flag_cycle: 0,
            blip,
            blip_clock: 0,
            prev_blip_output: 0,
            sample_queue: VecDeque::with_capacity(BLIP_BUFFER_SIZE as usize),
        }
    }

    pub fn connect_cpu(&mut self, _cpu: Rc<RefCell<Cpu>>) {}

    pub fn read_register(&mut self, address: u16, peek: bool) -> u8 {
        if address != 0x4015 {
            return 0xFF;
        }
        if peek {
            return 0xFF;
        }
        let mut value = 0;
        value.set_flag_enabled(
            status_register::ENABLE_PULSE1,
            self.pulse1.is_length_counter_non_zero(),
        );
        value.set_flag_enabled(
            status_register::ENABLE_PULSE2,
            self.pulse2.is_length_counter_non_zero(),
        );
        value.set_flag_enabled(status_register::FRAME_INTERRUPT, self.frame_interrupt_flag);
        self.frame_interrupt_flag = false;
        self.sync_irq_line();
        value
    }

    pub fn write_register(&mut self, address: u16, value: u8) {
        match address {
            0x4000..0x4004 => self.pulse1.write_register(address, value),
            0x4004..0x4008 => self.pulse2.write_register(address, value),
            0x4008..0x400C => self.triangle.write_register(address, value),
            0x4015 => {
                self.pulse1
                    .set_enabled(value.get_flag_enabled(status_register::ENABLE_PULSE1));
                self.pulse2
                    .set_enabled(value.get_flag_enabled(status_register::ENABLE_PULSE2));
                self.triangle
                    .set_enabled(value.get_flag_enabled(status_register::ENABLE_TRIANGLE));
            }
            0x4017 => {
                self.interrupt_inhibit_flag =
                    value.get_flag_enabled(frame_counter_register::INTERRUPT_INHIBIT);
                if self.interrupt_inhibit_flag {
                    self.frame_interrupt_flag = false;
                    self.sync_irq_line();
                }
                self.new_mode_flag = value.get_flag_enabled(frame_counter_register::SEQUENCER_MODE);
                let offset = if self.cpu_total_cycles % 2 == 0 { 3 } else { 4 };
                self.new_mode_flag_cycle = self.cpu_total_cycles + offset;
            }
            _ => (),
        }
    }

    // TODO: fix this later
    fn sync_irq_line(&mut self) {}

    /// https://www.nesdev.org/wiki/APU_Mixer
    fn mix(&mut self) -> f32 {
        let pulse1 = self.pulse1.next().unwrap();
        let pulse2 = self.pulse2.next().unwrap();

        let pulse_out = if pulse1 + pulse2 == 0 {
            0.0
        } else {
            95.88 / ((8128.0 / (pulse1 as f32 + pulse2 as f32)) + 100.0)
        };

        let triangle = self.triangle.next().unwrap();
        let noise: u8 = 0;
        let dmc: u8 = 0;

        let tnd_out = if triangle + noise + dmc == 0 {
            0.0
        } else {
            159.79
                / (1.0
                    / ((triangle as f32 / 8227.0)
                        + (noise as f32 / 12241.0)
                        + (dmc as f32 / 22638.0))
                    + 100.0)
        };

        pulse_out + tnd_out
    }

    fn feed_blip(&mut self) {
        let output_i32 = (self.mix() * BLIP_SCALE) as i32;
        let delta = output_i32 - self.prev_blip_output;

        if delta != 0 {
            self.blip.add_delta(self.blip_clock, delta);
            self.prev_blip_output = output_i32;
        }

        self.blip_clock += 1;

        if self.blip_clock >= BLIP_FRAME_SIZE {
            self.drain_blip();
        }
    }

    fn drain_blip(&mut self) {
        self.blip.end_frame(self.blip_clock);
        self.blip_clock = 0;

        let count = self.blip.samples_avail() as usize;
        if count == 0 {
            return;
        }

        let mut buf = vec![0i16; count];
        self.blip.read_samples(&mut buf, false);

        for s in buf {
            self.sample_queue.push_back(s as f32 / BLIP_SCALE);
        }
    }

    pub fn tick(&mut self) {
        let is_apu_cycle = self.cpu_total_cycles % 2 == 0;
        let mut immediate_frame_clock = false;

        if self.cpu_total_cycles == self.new_mode_flag_cycle {
            self.sequencer_mode_flag = self.new_mode_flag;
            self.apu_total_cycles = 0;
            immediate_frame_clock = self.sequencer_mode_flag;
        }

        if is_apu_cycle {
            self.apu_total_cycles += 1;
        }

        if !self.sequencer_mode_flag {
            if self.apu_total_cycles >= 14915 {
                self.apu_total_cycles = 0;
            }
        } else {
            if self.apu_total_cycles >= 18641 {
                self.apu_total_cycles = 0;
            }
        }

        #[rustfmt::skip]
        let mut apu_tick = if is_apu_cycle {
            ApuTick::default()
        } else if !self.sequencer_mode_flag {
            match self.apu_total_cycles {
                3728  => ApuTick { is_quarter_frame: true, is_half_frame: false, ..ApuTick::default() },
                7456  => ApuTick { is_quarter_frame: true, is_half_frame: true,  ..ApuTick::default() },
                11185 => ApuTick { is_quarter_frame: true, is_half_frame: false, ..ApuTick::default() },
                14914 => ApuTick { is_quarter_frame: true, is_half_frame: true,  ..ApuTick::default() },
                _     => ApuTick::default(),
            }
        } else {
            match self.apu_total_cycles {
                3728  => ApuTick { is_quarter_frame: true, is_half_frame: false, ..ApuTick::default() },
                7456  => ApuTick { is_quarter_frame: true, is_half_frame: true,  ..ApuTick::default() },
                11185 => ApuTick { is_quarter_frame: true, is_half_frame: false, ..ApuTick::default() },
                18640 => ApuTick { is_quarter_frame: true, is_half_frame: true,  ..ApuTick::default() },
                _     => ApuTick::default(),
            }
        };

        apu_tick.is_apu_cycle = is_apu_cycle;
        if immediate_frame_clock {
            apu_tick.is_quarter_frame = true;
            apu_tick.is_half_frame = true;
        }

        if !self.sequencer_mode_flag
            && !self.interrupt_inhibit_flag
            && (self.apu_total_cycles == 14914 || (self.apu_total_cycles == 0 && is_apu_cycle))
        {
            self.frame_interrupt_flag = true;
        }
        self.sync_irq_line();

        self.pulse1.tick(apu_tick);
        self.pulse2.tick(apu_tick);
        self.triangle.tick(apu_tick);

        self.feed_blip();

        self.cpu_total_cycles += 1;
    }
}

impl Iterator for Apu {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        self.sample_queue.pop_front()
    }
}
