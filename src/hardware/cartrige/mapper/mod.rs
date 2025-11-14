use crate::hardware::cartrige::{Header, error::CartrigeParseError, mapper::implementations::*};

use super::Result;

mod implementations;

pub(super) trait Mapper {
    fn map_write(&mut self, address: u16, value: u8) -> u16;
    fn map_read(&self, address: u16) -> u16;
}

pub(super) fn from_header(header: Header) -> Result<Box<dyn Mapper>> {
    match header.get_mapper_id() {
        0 => Ok(Box::new(M000 {header})),
        unkown_id => Err(CartrigeParseError::UnknownMapperIdError(unkown_id)),
    }
}
