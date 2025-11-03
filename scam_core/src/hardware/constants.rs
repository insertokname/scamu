pub const BUS_SIZE: usize = 2 << 16;

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
