use crate::hardware::{bus::Bus, constants, cpu::instructions::INSTRUCTIONS_LOOKUP};

mod addressing_modes;
mod instructions;
mod operations;

pub struct Cpu {
    accumulator: u8,
    x: u8,
    y: u8,
    program_counter: u16,
    stack_pointer: u8,
    status: u8,
    cycles_left: u8,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            accumulator: 0,
            x: 0,
            y: 0,
            program_counter: 0,
            stack_pointer: 0xFD,
            status: constants::cpu_flags::UNUSED,
            cycles_left: 0,
        }
    }

    pub fn reset(&mut self, bus: &Bus) {
        *self = Self::new();
        self.program_counter = bus.read_u16(0xFFFC);
    }

    pub fn set_flag(&mut self, flag: u8, enabled: bool) {
        if enabled {
            self.status |= flag;
        } else {
            self.status &= !flag;
        }
    }

    pub fn get_flag(&self, flag: u8) -> bool {
        (self.status & flag) > 0
    }

    pub fn push_stack(&mut self, value: u8, bus: &mut Bus) {
        bus.write(0x100 + self.stack_pointer as u16, value);
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
    }

    pub fn pop_stack(&mut self, bus: &Bus) -> u8 {
        self.stack_pointer = self.stack_pointer.wrapping_add(1);
        bus.read(0x100 + self.stack_pointer as u16)
    }

    pub fn push_stack_u16(&mut self, value: u16, bus: &mut Bus) {
        let high = (value >> 8) as u8;
        let low = value as u8;

        self.push_stack(high, bus);
        self.push_stack(low, bus);
    }

    pub fn pop_stack_u16(&mut self, bus: &Bus) -> u16 {
        let low = self.pop_stack(bus) as u16;
        let high = self.pop_stack(bus) as u16;
        (high << 8) | low
    }

    pub fn get_cycles_left(&self) -> u8 {
        self.cycles_left
    }

    pub fn tick(&mut self, bus: &mut Bus) {
        if self.cycles_left > 0 {
            self.cycles_left -= 1;
        } else {
            let instruction_location = self.program_counter;
            let instruction_code = bus.read(self.program_counter);
            
            self.program_counter += 1;

            let mut next_instruction =
                (&INSTRUCTIONS_LOOKUP[instruction_code as usize]).create(self, bus);

            println!(
                "Exectuing instruction {} at address {:#X}",
                next_instruction.dissassemble_instruction(self, bus),
                instruction_location
            );

            let required_cycles = next_instruction.execute(self, bus);
            self.cycles_left += required_cycles;

            // We treat this whole loading and executing as 1 cycle
            // and the other ones are stored in the left_cycles and we
            // will artificially drain the left_cycles in the next ticks
            self.cycles_left -= 1;
        }
    }
}
