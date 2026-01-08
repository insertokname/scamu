use std::{cell::RefCell, rc::Rc};

use crate::hardware::{cartrige::Cartrige, cpu::Cpu, cpu_bus::CpuBus};

pub struct Nes {
    bus: CpuBus,
    cpu: Cpu,
    cartrige: Option<Rc<RefCell<Cartrige>>>,
}

impl Nes {
    pub fn new() -> Self {
        Self {
            bus: CpuBus::new(),
            cpu: Cpu::new(),
            cartrige: None,
        }
    }

    pub fn insert_cartrige(&mut self, cartrige: Cartrige) {
        let cartrige = Rc::new(RefCell::new(cartrige));
        self.bus.insert_cartrige(cartrige.clone());
        self.cartrige = Some(cartrige);
    }

    pub fn is_resetting(&self) -> bool {
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
