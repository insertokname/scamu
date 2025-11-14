#[macro_export]
macro_rules! byte_size {
    ($val:literal b) => {
        $val
    };
    ($val:literal kb) => {
        $val * 1024usize
    };
    ($val:literal mb) => {
        $val * 1024usize * 1024
    };
    ($val:literal gb) => {
        $val * 1024usize * 1024 * 1024
    };
    ($val:literal tb) => {
        $val * 1024usize * 1024 * 1024 * 1024 * 1024
    };
}

pub const BUS_SIZE: usize = byte_size!(64 kb);
pub const CPU_RAM_SIZE: usize = byte_size!(2 kb);

pub const STACK_OFFSET: u16 = 0x100;

pub const NES_MAGIC_NUMBERS: [u8; 4] = [0x4E, 0x45, 0x53, 0x1A];
pub const PRG_ROM_BANK_SIZE: usize = byte_size!(16 kb);
pub const CHR_ROM_BANK_SIZE: usize = byte_size!(8 kb);
pub const PRG_RAM_BANK_SIZE: usize = byte_size!(8 kb);

pub const FLAG6_NAMETABLE: u8 = 1 << 0;
pub const FLAG6_BATTERY: u8 = 1 << 1;
pub const FLAG6_TRAINER: u8 = 1 << 2;
pub const FLAG6_FOUR_SCREEN: u8 = 1 << 3;
pub const FLAG7_VS_UNISYSTEM: u8 = 1 << 0;
pub const FLAG7_PLAYCHOICE_10: u8 = 1 << 1;
pub const FLAG7_NES2_SIGNATURE_MASK: u8 = (1 << 3) | (1 << 2);
pub const FLAG7_NES2_SIGNATURE_VALUE: u8 = 1 << 3;
pub const FLAG9_TV_SYSTEM: u8 = 1 << 0;
pub const FLAG10_TV_SYSTEM_MASK: u8 = (1 << 1) | (1 << 0);

#[rustfmt::skip]
pub mod cpu_flags {
    pub const CARRY             :u8 = 0b00000001;
    pub const ZERO              :u8 = 0b00000010;
    pub const INTERRUPT_DISABLE :u8 = 0b00000100;
    pub const DECIMAL_MODE      :u8 = 0b00001000;
    pub const BREAK             :u8 = 0b00010000;
    pub const UNUSED            :u8 = 0b00100000;
    pub const OVERFLOW          :u8 = 0b01000000;
    pub const NEGATIVE          :u8 = 0b10000000;
}
