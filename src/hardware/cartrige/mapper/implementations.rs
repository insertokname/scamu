use crate::hardware::cartrige::{Header, Mapper};

pub(super) struct M000 {
    pub header: Header,
}

impl Mapper for M000 {
    fn map_write(&mut self, address: u16, _: u8) -> u16 {
        self.map_prg_address(address)
    }

    fn map_read(&self, address: u16) -> u16 {
        self.map_prg_address(address)
    }
}

impl M000 {
    fn map_prg_address(&self, address: u16) -> u16 {
        let offset = address - 0x8000;
        if self.header.prg_size == 1 {
            offset & 0x3FFF
        } else {
            offset
        }
    }
}
