use crate::hardware::{bus::Bus, cpu::Cpu};

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

    pub fn reset(&mut self) {
        self.cpu.reset(&self.bus);
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
