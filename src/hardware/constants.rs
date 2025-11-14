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
