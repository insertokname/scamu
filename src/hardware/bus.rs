use super::constants;

pub struct Bus {
    memory: [u8; constants::BUS_SIZE],
}

impl Bus {
    pub fn new() -> Self {
        Self {
            memory: [0; constants::BUS_SIZE],
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    pub fn write(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value
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
