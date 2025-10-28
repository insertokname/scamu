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

    pub fn read(&self, index: u16) -> u8 {
        self.memory[index as usize]
    }

    pub fn write(&mut self, index: u16, value: u8) {
        self.memory[index as usize] = value
    }
}
