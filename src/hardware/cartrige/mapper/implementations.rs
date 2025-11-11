use crate::hardware::cartrige::Mapper;

pub(super) struct M000 {}

impl Mapper for M000 {
    fn map_write(&mut self, address: u16, value: u8) -> u16 {
        address
    }

    // TODO: actually do the mirroring logic for the correct banks
    fn map_read(&self, address: u16) -> u16 {
        address & 0x3FFF
    }
}
