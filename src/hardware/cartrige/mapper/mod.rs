use crate::hardware::cartrige::mapper::implementations::*;

mod implementations;

pub(super) trait Mapper {
    fn map_write(&mut self, address: u16, value: u8) -> u16;
    fn map_read(&self, address: u16) -> u16;
}

pub(super) fn from_id(id: u8) -> Box<dyn Mapper> {
    Box::new(M000 {})
}
