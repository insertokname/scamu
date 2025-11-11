#![allow(dead_code, unused_variables)]

pub mod error;
mod mapper;

use crate::hardware::cartrige::{
    error::CartrigeParseError,
    mapper::{Mapper, from_id},
};

pub type Result<T> = std::result::Result<T, CartrigeParseError>;

const NES_MAGIC_NUMBERS: [u8; 4] = [0x4E, 0x45, 0x53, 0x1A];

fn try_get_next_n<'a>(data_ptr: &mut &'a [u8], n: usize) -> Result<&'a [u8]> {
    if data_ptr.len() < n {
        return Err(CartrigeParseError::NotEnoughBytesError(n));
    } else {
        let start = data_ptr.get(0..n);
        *data_ptr = &data_ptr[n..];
        start.ok_or_else(|| CartrigeParseError::NotEnoughBytesError(n))
    }
}

fn try_get_next(data_ptr: &mut &[u8]) -> Result<u8> {
    if data_ptr.len() < 1 {
        return Err(CartrigeParseError::NotEnoughBytesError(1));
    } else {
        let start = data_ptr.get(0);
        *data_ptr = &data_ptr[1..];
        start
            .cloned()
            .ok_or_else(|| CartrigeParseError::NotEnoughBytesError(1))
    }
}

pub struct Cartrige {
    mapper: Box<dyn Mapper>,
    header: Header,
    prg_mem: Vec<u8>,
    chr_mem: Vec<u8>,
}

impl Cartrige {
    pub fn from_file(filename: &str) -> Result<Self> {
        let bytes = std::fs::read(filename)?;
        Cartrige::from_bytes(bytes.as_slice())
    }

    pub fn from_raw_prg_mem(bytes: &[u8]) -> Result<Self> {
        if bytes.len() > 16384 {
            return Err(CartrigeParseError::RawProgramMemoryToLargeError(
                bytes.len(),
            ));
        }

        let header = Header {
            prg_size: 1,
            chr_size: 0,
            flag6: 0,
            flag7: 0,
        };

        Ok(Self {
            mapper: from_id(0),
            chr_mem: vec![],
            header,
            prg_mem: bytes.to_vec(),
        })
    }

    pub fn from_bytes(mut bytes: &[u8]) -> Result<Self> {
        let bytes_ptr: &mut &[u8] = &mut bytes;

        if try_get_next_n(bytes_ptr, 4)? != &NES_MAGIC_NUMBERS {
            println!("test");
        }

        let prg_size = try_get_next(bytes_ptr)?;
        let chr_size = try_get_next(bytes_ptr)?;
        let flag6 = try_get_next(bytes_ptr)?;
        let flag7 = try_get_next(bytes_ptr)?;
        let _flag8 = try_get_next(bytes_ptr)?;
        let _flag9 = try_get_next(bytes_ptr)?;
        let _flag10 = try_get_next(bytes_ptr)?;
        let _ = try_get_next_n(bytes_ptr, 5)?;

        let header = Header {
            prg_size,
            chr_size,
            flag6,
            flag7,
        };

        if header.get_has_trainer() {
            let _ = try_get_next_n(bytes_ptr, 512)?;
        }

        let prg_mem = try_get_next_n(bytes_ptr, 16384 * prg_size as usize)?.to_vec();
        let chr_mem = try_get_next_n(bytes_ptr, 8192 * chr_size as usize)?.to_vec();

        // TODO: implement mapper logic
        let mapper = mapper::from_id(0);

        Ok(Self {
            mapper,
            header,
            prg_mem,
            chr_mem,
        })
    }

    // TODO: impl reading from chr or prg mem
    pub fn write(&mut self, address: u16, value: u8) {
        let _ = self.mapper.map_write(address, value);
    }

    pub fn read(&self, address: u16) -> u8 {
        let addr = self.mapper.map_read(address);
        self.prg_mem[addr as usize]
    }
}

// TODO: complete header
pub struct Header {
    prg_size: u8,
    chr_size: u8,
    flag6: u8,
    flag7: u8,
}

impl Header {
    pub fn get_mapper_id(&self) -> u8 {
        ((self.flag6 >> 4) << 4) | (self.flag7 >> 4)
    }

    pub fn get_has_trainer(&self) -> bool {
        self.flag6 & 0x4 > 0
    }
}
