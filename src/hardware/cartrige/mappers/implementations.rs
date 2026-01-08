use crate::{
    byte_size,
    hardware::cartrige::{Header, Mapper, memory_access::MemoryAccess},
};

pub(super) struct M000 {
    pub header: Header,
}

impl Mapper for M000 {
    fn new(header: Header) -> Self
    where
        Self: Sized,
    {
        Self { header }
    }

    fn map_read(&mut self, memory_access: MemoryAccess) -> Option<u16> {
        match memory_access {
            MemoryAccess::CpuAccess { address } if address < 0x8000 => None,
            MemoryAccess::CpuAccess { address } => {
                let offset = address - 0x8000;
                if self.header.prg_rom_size() == 1 {
                    Some(offset & 0x3FFF)
                } else {
                    Some(offset)
                }
            }
            MemoryAccess::PpuAccess { address } if address < 0x2000 => Some(address),
            MemoryAccess::PpuAccess { .. } => None,
        }
    }

    fn map_write(&mut self, memory_access: MemoryAccess, _: u8) -> Option<u16> {
        match memory_access {
            MemoryAccess::CpuAccess { .. } => None,
            MemoryAccess::PpuAccess { address } if address < 0x2000 => {
                if self.header.chr_size == 0 {
                    Some(address)
                } else {
                    None
                }
            }
            MemoryAccess::PpuAccess { .. } => None,
        }
    }
}

pub(super) struct M002 {
    pub header: Header,
    selected_bank: u8,
}

impl Mapper for M002 {
    fn new(header: Header) -> Self
    where
        Self: Sized,
    {
        Self {
            header,
            selected_bank: 0,
        }
    }

    fn map_read(&mut self, memory_access: MemoryAccess) -> Option<u16> {
        match memory_access {
            MemoryAccess::CpuAccess { address } if address < 0x8000 => None,
            MemoryAccess::CpuAccess { address } if address < 0xC000 => {
                Some((self.selected_bank) as u16 * byte_size!(16 kb) as u16 + (address & 0x3FFF))
            }
            MemoryAccess::CpuAccess { address } => Some(
                (self.header.prg_rom_size() - 1) as u16 * byte_size!(16 kb) as u16
                    + (address & 0x3FFF),
            ),
            MemoryAccess::PpuAccess { address } if address < 0x2000 => Some(address),
            MemoryAccess::PpuAccess { .. } => None,
        }
    }

    fn map_write(&mut self, memory_access: MemoryAccess, value: u8) -> Option<u16> {
        match memory_access {
            MemoryAccess::CpuAccess { address } if address < 0x8000 => None,
            MemoryAccess::CpuAccess { .. } => {
                self.selected_bank = value & 0x0F;
                None
            }
            MemoryAccess::PpuAccess { address } if address < 0x2000 => {
                if self.header.chr_size == 0 {
                    Some(address)
                } else {
                    None
                }
            }
            MemoryAccess::PpuAccess { .. } => None,
        }
    }
}
