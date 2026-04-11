use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

use crate::hardware::{
    cartrige::{Cartrige, cartrige_access::CartrigeAccess},
    ppu::Ppu,
};

use super::constants;

pub struct CpuBus {
    cpu_ram: [u8; constants::cpu::RAM_SIZE],
    cartrige: Option<Rc<RefCell<Cartrige>>>,
    ppu: Option<Rc<RefCell<Ppu>>>,
    last_read: Cell<u8>,
}

impl CpuBus {
    pub fn new() -> Self {
        Self {
            cpu_ram: [0; constants::cpu::RAM_SIZE],
            cartrige: None,
            ppu: None,
            last_read: Cell::new(0),
        }
    }

    pub fn insert_cartrige(&mut self, cartrige: Rc<RefCell<Cartrige>>) {
        self.cartrige = Some(cartrige);
    }

    pub fn connect_ppu(&mut self, ppu: Rc<RefCell<Ppu>>) {
        self.ppu = Some(ppu);
    }

    pub fn read(&self, address: u16) -> u8 {
        self.read_inner(address, false)
    }

    /// Same as [CpuBus::read] but doesn't mutate state (used here for debugging)
    pub fn peek(&self, address: u16) -> u8 {
        self.read_inner(address, true)
    }

    pub(crate) fn read_inner(&self, address: u16, peek: bool) -> u8 {
        let result = match address {
            0x0..0x2000 => self.cpu_ram[address as usize & (constants::cpu::RAM_SIZE - 1)],
            0x2000..0x4000 => self
                .ppu
                .as_ref()
                .map(|c| c.borrow_mut().read_register_inner(address, peek))
                .unwrap_or(0),
            0x4000..0x4020 => 0xFF, // TODO: impl apu
            0x4020.. => self
                .cartrige
                .as_ref()
                .map(|c| {
                    c.borrow_mut()
                        .read(CartrigeAccess::CpuAccess { address })
                        .unwrap_or_else(|| self.last_read.get())
                })
                .unwrap_or(0x0),
        };
        self.last_read.set(result);
        return result;
    }

    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            0x0..0x2000 => self.cpu_ram[address as usize & (constants::cpu::RAM_SIZE - 1)] = value,
            0x2000..0x4000 => self
                .ppu
                .as_ref()
                .map(|c| c.borrow_mut().write_register(address, value))
                .unwrap_or(()),
            0x4000..0x4020 => (), // TODO: impl apu
            0x4020.. => self
                .cartrige
                .as_ref()
                .map(|c| {
                    c.borrow_mut()
                        .write(CartrigeAccess::CpuAccess { address }, value)
                })
                .unwrap_or(()),
        }
    }

    pub fn read_u16(&self, address: u16) -> u16 {
        let pointer_low = self.read(address) as u16;
        let pointer_high = self.read(address + 1) as u16;
        pointer_high << 8 | pointer_low
    }

    pub fn peek_u16(&self, address: u16) -> u16 {
        let pointer_low = self.peek(address) as u16;
        let pointer_high = self.peek(address + 1) as u16;
        pointer_high << 8 | pointer_low
    }

    pub fn write_u16(&mut self, address: u16, value: u16) {
        let value_low = (value & 0x00FF) as u8;
        let value_high = (value >> 8) as u8;

        self.cpu_ram[address as usize] = value_low;
        self.cpu_ram[(address + 1) as usize] = value_high;
    }

    pub fn write_memory(&mut self, start: u16, memory: &[u8]) {
        for i in 0..memory.len() {
            self.write(start + i as u16, memory[i]);
        }
    }
}
