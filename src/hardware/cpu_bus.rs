use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

use crate::hardware::cartrige::{Cartrige, memory_access::MemoryAccess};

use super::constants;

pub struct CpuBus {
    cpu_ram: [u8; constants::CPU_RAM_SIZE],
    cartrige: Option<Rc<RefCell<Cartrige>>>,
    last_read: Cell<u8>,
}

impl CpuBus {
    pub fn new() -> Self {
        Self {
            cpu_ram: [0; constants::CPU_RAM_SIZE],
            cartrige: None,
            last_read: Cell::new(0),
        }
        // // used to pass nestest. will be implemented once APU is ok
        // for addr in 0x4000..0x4020 {
        //     bus.cpu_ram[addr] = 0xFF;
        // }
        // bus
    }

    pub fn insert_cartrige(&mut self, cartrige: Rc<RefCell<Cartrige>>) {
        self.cartrige = Some(cartrige);
    }

    pub fn read(&self, address: u16) -> u8 {
        let result = match address {
            0x0..0x2000 => self.cpu_ram[address as usize & (constants::CPU_RAM_SIZE - 1)],
            0x2000..0x4000 => 0,    //TODO: impl ppu registers
            0x4000..0x4020 => 0xFF, // TODO: impl apu
            0x4020.. => self
                .cartrige
                .as_ref()
                .map(|c| {
                    c.borrow_mut()
                        .read(MemoryAccess::CpuAccess { address })
                        .unwrap_or_else(|| self.last_read.get())
                })
                .unwrap_or(0x0),
        };
        self.last_read.set(result);
        return result;
    }

    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            0x0..0x2000 => self.cpu_ram[address as usize & (constants::CPU_RAM_SIZE - 1)] = value,
            0x2000..0x4000 => (), //TODO: impl ppu registers
            0x4000..0x4020 => (), // TODO: impl apu
            0x4020.. => self
                .cartrige
                .as_ref()
                .map(|c| {
                    c.borrow_mut()
                        .write(MemoryAccess::CpuAccess { address }, value)
                })
                .unwrap_or(()),
        }
    }

    pub fn read_u16(&self, address: u16) -> u16 {
        let pointer_low = self.read(address) as u16;
        let pointer_high = self.read(address + 1) as u16;
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
