use funty::Unsigned;

#[derive(Clone, Copy, Default, Debug)]
pub struct BitFlags<T: Unsigned> {
    pub value: T,
}

impl<T: Unsigned> BitFlags<T> {
    pub fn new(value: T) -> Self {
        Self { value }
    }

    pub fn set_bitfield(&mut self, field: T, value: T) {
        self.value &= !field;
        self.value |= (value << field.trailing_zeros()) & field;
    }

    pub fn get_bitfield(&self, field: T) -> T {
        (self.value & field) >> field.trailing_zeros()
    }

    pub fn set_bitmasked(&mut self, mask: T, value: T) {
        self.value &= !mask;
        self.value |= value & mask;
    }

    pub fn get_bitmasked(&self, mask: T) -> T {
        self.value & mask
    }

    pub fn set_flag_enabled(&mut self, flag: T, enabled: bool) {
        if enabled {
            self.value |= flag;
        } else {
            self.value &= !flag;
        }
    }

    pub fn get_flag_enabled(&self, flag: T) -> bool {
        (self.value & flag) == flag
    }
}

pub trait BitOps: Unsigned {
    fn set_bitfield(&mut self, field: Self, value: Self) {
        *self &= !field;
        *self |= (value << field.trailing_zeros()) & field;
    }

    fn get_bitfield(self, field: Self) -> Self {
        (self & field) >> field.trailing_zeros()
    }

    fn set_bitmasked(&mut self, mask: Self, value: Self) {
        *self &= !mask;
        *self |= value & mask;
    }

    fn get_bitmasked(self, mask: Self) -> Self {
        self & mask
    }

    fn set_flag_enabled(&mut self, flag: Self, enabled: bool) {
        if enabled {
            *self |= flag;
        } else {
            *self &= !flag;
        }
    }

    fn get_flag_enabled(self, flag: Self) -> bool {
        (self & flag) == flag
    }
}

impl<T: Unsigned> BitOps for T {}
