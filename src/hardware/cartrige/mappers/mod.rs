use crate::hardware::cartrige::{
    Header, error::CartrigeParseError, mappers::implementations::*, memory_access::MemoryAccess,
};

use super::Result;

mod implementations;

pub(super) trait Mapper {
    fn new(header: Header) -> Self
    where
        Self: Sized;
    fn map_write(&mut self, memory_access: MemoryAccess, value: u8) -> Option<u16>;
    fn map_read(&mut self, memory_access: MemoryAccess) -> Option<u16>;
}

pub(super) fn from_header(header: Header) -> Result<Box<dyn Mapper>> {
    Ok(match header.get_mapper_id() {
        0 => Box::new(M000::new(header)),
        2 => Box::new(M002::new(header)),
        unkown_id => return Err(CartrigeParseError::UnknownMapperIdError(unkown_id)),
    })
}
