use crate::hardware::{bus::Bus, cartrige::Cartrige, cpu::Cpu};

pub struct Nes {
    bus: Bus,
    cpu: Cpu,
}

impl Nes {
    pub fn new() -> Self {
        Self {
            bus: Bus::new(),
            cpu: Cpu::new(),
        }
    }

    pub fn insert_cartrige(&mut self, cartrige: Cartrige) {
        self.bus.insert_cartrige(cartrige);
    }

    pub fn is_resetting(&self) -> bool{
        self.cpu.is_resetting()
    }

    pub fn reset(&mut self) {
        self.cpu.reset(&self.bus);
    }

    pub fn reset_with_program_counter(&mut self, program_counter: u16) {
        self.cpu.reset_with_program_counter(program_counter);
    }

    pub fn tick(&mut self) {
        self.cpu.tick(&mut self.bus);
    }

    pub fn write_memory(&mut self, start: u16, memory: &[u8]) {
        for i in 0..memory.len() {
            self.bus.write(start + i as u16, memory[i]);
        }
    }
}
