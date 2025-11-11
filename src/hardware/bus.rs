use super::constants;
use crate::hardware::cartrige::Cartrige;

pub struct Bus {
    memory: [u8; constants::BUS_SIZE],
    cartrige: Option<Cartrige>,
}

impl Bus {
    pub fn new() -> Self {
        let mut bus = Self {
            memory: [0; constants::BUS_SIZE],
            cartrige: None,
        };
        for addr in 0x4000..0x4020 {
            bus.memory[addr] = 0xFF;
        }
        bus
    }

    pub fn insert_cartrige(&mut self, cartrige: Cartrige) {
        self.cartrige = Some(cartrige);
    }

    pub fn read(&self, address: u16) -> u8 {
        match address {
            0..0x8000 => self.memory[address as usize],
            0x8000.. => match self.cartrige.as_ref() {
                Some(some) => some.read(address),
                None => 0,
            },
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            0..0x8000 => self.memory[address as usize] = value,
            0x8000.. => {
                if let Some(some) = self.cartrige.as_mut() {
                    some.write(address, value)
                }
            }
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

        self.memory[address as usize] = value_low;
        self.memory[(address + 1) as usize] = value_high;
    }

    pub fn write_memory(&mut self, start: u16, memory: &[u8]) {
        for i in 0..memory.len() {
            self.write(start + i as u16, memory[i]);
        }
    }
}
