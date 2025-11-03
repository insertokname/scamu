use std::fmt::Debug;

use crate::hardware::{bus::Bus, cpu::Cpu};

pub(super) trait AddressingMode<T: Debug> {
    fn additional_cycles_required(&self) -> u8;
    fn requires_another_cycle(&mut self);
    fn read(&self, cpu: &Cpu, bus: &Bus) -> T;
    fn write(&mut self, new_value: T, cpu: &mut Cpu, bus: &mut Bus);
}

pub(super) type AddressingModeFactory<T> =
    fn(cpu: &mut Cpu, bus: &mut Bus) -> Box<dyn AddressingMode<T>>;

struct ImplicitAddressingMode {
    additional_cycles_required: u8,
}

impl AddressingMode<()> for ImplicitAddressingMode {
    fn additional_cycles_required(&self) -> u8 {
        self.additional_cycles_required
    }

    fn requires_another_cycle(&mut self) {
        self.additional_cycles_required += 1
    }

    fn read(&self, _: &Cpu, _: &Bus) -> () {
        ()
    }

    fn write(&mut self, _: (), _: &mut Cpu, _: &mut Bus) {}
}

struct AccumulatorAddressingMode {
    additional_cycles_required: u8,
}

impl AddressingMode<u8> for AccumulatorAddressingMode {
    fn additional_cycles_required(&self) -> u8 {
        self.additional_cycles_required
    }

    fn requires_another_cycle(&mut self) {
        self.additional_cycles_required += 1
    }

    fn read(&self, cpu: &Cpu, _: &Bus) -> u8 {
        cpu.accumulator
    }

    fn write(&mut self, new_value: u8, cpu: &mut Cpu, _: &mut Bus) {
        cpu.accumulator = new_value;
    }
}

struct MemoryAddressingMode {
    address: u16,
    additional_cycles_required: u8,
}

impl AddressingMode<u8> for MemoryAddressingMode {
    fn additional_cycles_required(&self) -> u8 {
        self.additional_cycles_required
    }

    fn requires_another_cycle(&mut self) {
        self.additional_cycles_required += 1
    }

    fn read(&self, _: &Cpu, bus: &Bus) -> u8 {
        bus.read(self.address)
    }

    fn write(&mut self, new_value: u8, _: &mut Cpu, bus: &mut Bus) {
        bus.write(self.address, new_value);
    }
}

struct RelativeAddressingMode {
    address: u16,
    additional_cycles_required: u8,
}

impl AddressingMode<i8> for RelativeAddressingMode {
    fn additional_cycles_required(&self) -> u8 {
        self.additional_cycles_required
    }

    fn requires_another_cycle(&mut self) {
        self.additional_cycles_required += 1
    }

    fn read(&self, _: &Cpu, bus: &Bus) -> i8 {
        bus.read(self.address) as i8
    }

    fn write(&mut self, new_value: i8, _: &mut Cpu, bus: &mut Bus) {
        bus.write(self.address, new_value as u8);
    }
}

/// Gives the user access to both the address and the value at the address
#[derive(Clone, Copy)]
pub(super) struct JumpAddress {
    value: u8,
    address: u16,
}

impl Debug for JumpAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[allow(dead_code)]
impl JumpAddress {
    pub fn get_value(&self) -> u8 {
        self.value
    }

    pub fn set_value(&mut self, new_value: u8) {
        self.value = new_value
    }

    pub fn get_address(&self) -> u16 {
        self.address
    }
}

struct JumpingAddressingMode {
    address: u16,
    additional_cycles_required: u8,
}

impl AddressingMode<JumpAddress> for JumpingAddressingMode {
    fn additional_cycles_required(&self) -> u8 {
        self.additional_cycles_required
    }

    fn requires_another_cycle(&mut self) {
        self.additional_cycles_required += 1
    }
    fn read(&self, _: &Cpu, bus: &Bus) -> JumpAddress {
        JumpAddress {
            value: bus.read(self.address),
            address: self.address,
        }
    }

    fn write(&mut self, new_value: JumpAddress, _: &mut Cpu, bus: &mut Bus) {
        bus.write(self.address, new_value.value);
    }
}

// pub(super) type AddressingMode<T> = fn(cpu: &mut Cpu, bus: &mut Bus) -> CycleEffect<T>;

// /// Implicit addressing mode
// ///
// /// Instructions using implicit mode do not require a parameter (ex: CLC)
pub(super) const IMPLICIT: AddressingModeFactory<()> = |_cpu: &mut Cpu, _bus: &mut Bus| {
    Box::new(ImplicitAddressingMode {
        additional_cycles_required: 0,
    })
};

/// Accumulator addressing mode
///
/// Gets the acculumator as the argument
// pub(super) const ACCUMULATOR: AddressingMode<&mut u8> =
//     |cpu: &mut Cpu, _: &mut Bus| -> CycleEffect<&mut u8> {};
pub(super) const ACCUMULATOR: AddressingModeFactory<u8> =
    |_: &mut Cpu, _: &mut Bus| -> Box<dyn AddressingMode<u8>> {
        Box::new(AccumulatorAddressingMode {
            additional_cycles_required: 0,
        })
    };

/// Immediate addressing mode
///
/// Gets the next byte as the argument
pub(super) const IMMEDIATE: AddressingModeFactory<u8> = |cpu: &mut Cpu, _: &mut Bus| {
    let address = cpu.program_counter;
    cpu.program_counter += 1;

    Box::new(MemoryAddressingMode {
        address,
        additional_cycles_required: 0,
    })
};

/// Zero page addressing mode
///
/// Uses the next byte as a zero-page address (0x0000–0x00FF).
/// The CPU treats the operand as the low byte of the address and
/// assumes the high byte is 0x00.
///
/// # Example
///
/// LDA $42
///
/// Loads the value from memory at address 0x0042 into the accumulator
/// register.
pub(super) const ZERO_PAGE: AddressingModeFactory<u8> = |cpu: &mut Cpu, bus: &mut Bus| {
    let address = bus.read(cpu.program_counter) as u16;
    cpu.program_counter += 1;

    Box::new(MemoryAddressingMode {
        address,
        additional_cycles_required: 0,
    })
};

/// Zero page with x offset addressing mode
///
/// Uses the next byte + the x register as a zero-page address
/// (0x0000–0x00FF). The CPU treats the operand as the low byte of the
/// address and assumes the high byte is 0x00. The addition wraps around
/// within the zero page (i.e., (operand + X) & 0xFF).
///
/// # Example
///
/// LDA $42, X
///
/// Loads the value from memory at address 0x0042 + X into the accumulator
/// register.
pub(super) const ZERO_PAGE_X_OFFSET: AddressingModeFactory<u8> = |cpu: &mut Cpu, bus: &mut Bus| {
    let address = bus.read(cpu.program_counter).wrapping_add(cpu.x) as u16;
    cpu.program_counter += 1;

    Box::new(MemoryAddressingMode {
        address,
        additional_cycles_required: 0,
    })
};

/// Zero page with y offset addressing mode
///
/// Uses the next byte + the y register as a zero-page address
/// (0x0000–0x00FF). The CPU treats the operand as the low byte of the
/// address and assumes the high byte is 0x00. The addition wraps around
/// within the zero page (i.e., (operand + y) & 0xFF).
///
/// # Example
///
/// LDA $42, Y
///
/// Loads the value from memory at address 0x0042 + Y into the accumulator
/// register.
pub(super) const ZERO_PAGE_Y_OFFSET: AddressingModeFactory<u8> = |cpu: &mut Cpu, bus: &mut Bus| {
    let address = bus.read(cpu.program_counter).wrapping_add(cpu.y) as u16;
    cpu.program_counter += 1;

    Box::new(MemoryAddressingMode {
        address,
        additional_cycles_required: 0,
    })
};

/// Absolute addressing mode
///
/// Uses the next two bytes as the low and high parts of the target address,
/// allowing access to any location in memory.
///
/// # Example
///
/// LDA $1234
///
/// Loads the value from memory at address 0x1234 into the accumulator register.
pub(super) const ABSOLUTE: AddressingModeFactory<u8> = |cpu: &mut Cpu, bus: &mut Bus| {
    let address = bus.read_u16(cpu.program_counter);
    cpu.program_counter += 2;

    Box::new(MemoryAddressingMode {
        address,
        additional_cycles_required: 0,
    })
};

/// Absolute addressing mode
///
///
/// Used for jump instructions to allow them to also access the memory location
pub(super) const ABSOLUTE_JUMPING: AddressingModeFactory<JumpAddress> =
    |cpu: &mut Cpu, bus: &mut Bus| {
        let address = bus.read_u16(cpu.program_counter);
        cpu.program_counter += 2;

        Box::new(JumpingAddressingMode {
            address,
            additional_cycles_required: 0,
        })
    };

/// Absolute with x offset addressing mode
///
/// Uses the next two bytes as the low and high parts of the target
/// address then adds x to it.
///
/// # Example
///
/// LDA $1234, X
///
/// Loads the value from memory at address 0x1234 + X into the accumulator register.
pub(super) const ABSOLUTE_X_OFFSET: AddressingModeFactory<u8> = |cpu: &mut Cpu, bus: &mut Bus| {
    let address = bus.read_u16(cpu.program_counter);
    cpu.program_counter += 2;
    let offset_address = address + cpu.x as u16;

    let add_cycle = offset_address & 0xFF00 != address & 0xFF00;

    Box::new(MemoryAddressingMode {
        address: offset_address,
        additional_cycles_required: add_cycle as u8,
    })
};

/// Absolute with y offset addressing mode
///
/// Uses the next two bytes as the low and high parts of the target
/// address then adds y to it.
///
/// # Example
///
/// LDA $1234, Y
///
/// Loads the value from memory at address 0x1234 + Y into the accumulator register.
pub(super) const ABSOLUTE_Y_OFFSET: AddressingModeFactory<u8> = |cpu: &mut Cpu, bus: &mut Bus| {
    let address = bus.read_u16(cpu.program_counter);
    cpu.program_counter += 2;
    let offset_address = address + cpu.y as u16;

    let add_cycle = offset_address & 0xFF00 != address & 0xFF00;

    Box::new(MemoryAddressingMode {
        address: offset_address,
        additional_cycles_required: add_cycle as u8,
    })
};

// apparently this is only used for jumping and the INDIRECT_JUMPING already exists so wtv
// /// Indirect addressing mode
// ///
// /// Reads a 16-bit pointer from the next two bytes and returns the
// /// pointed-to address.
// pub(super) const INDIRECT: AddressingModeFactory<u8> = |cpu: &mut Cpu, bus: &mut Bus| {
//     let pointer_address = bus.read_u16(cpu.program_counter);
//     cpu.program_counter += 2;

//     let low = bus.read(pointer_address) as u16;

//     // bug in 6502 wrapping page https://www.nesdev.org/6502bugs.txt
//     // An indirect JMP (xxFF) will fail because the MSB will be fetched
//     // from address xx00 instead of page xx+1
//     let high_address = (pointer_address & 0xFF00) | ((pointer_address + 1) & 0x00FF);
//     let high = bus.read(high_address) as u16;
//     let address = (high << 8) | low;

//     Box::new(MemoryAddressingMode {
//         address,
//         additional_cycles_required: 0,
//     })
// };

/// Indirect addressing mode
///
/// Used for jump instructions to allow them to also access the memory location
pub(super) const INDIRECT_JUMPING: AddressingModeFactory<JumpAddress> =
    |cpu: &mut Cpu, bus: &mut Bus| {
        let pointer_address = bus.read_u16(cpu.program_counter);
        cpu.program_counter += 2;

        let low = bus.read(pointer_address) as u16;

        // bug in 6502 wrapping page https://www.nesdev.org/6502bugs.txt
        // An indirect JMP (xxFF) will fail because the MSB will be fetched
        // from address xx00 instead of page xx+1
        let high_address = (pointer_address & 0xFF00) | ((pointer_address + 1) & 0x00FF);
        let high = bus.read(high_address) as u16;
        let address = (high << 8) | low;

        Box::new(JumpingAddressingMode {
            address,
            additional_cycles_required: 0,
        })
    };

/// Indirect with x offset addressing mode
///
/// Reads an 8-bit pointer to a zero page location from the next byte + x
/// and then uses that as the actual address.
pub(super) const INDIRECT_X_OFFSET: AddressingModeFactory<u8> = |cpu: &mut Cpu, bus: &mut Bus| {
    let pointer_address = bus.read(cpu.program_counter) as u16;
    cpu.program_counter += 1;

    let address = bus.read_u16(pointer_address + cpu.x as u16);

    Box::new(MemoryAddressingMode {
        address,
        additional_cycles_required: 0,
    })
};

/// Indirect with y offset addressing mode
///
/// Reads an 8-bit pointer to a zero page location from the next byte
/// and then adds y to that loccation and returns that new address.
pub(super) const INDIRECT_Y_OFFSET: AddressingModeFactory<u8> = |cpu: &mut Cpu, bus: &mut Bus| {
    let pointer_address = bus.read(cpu.program_counter) as u16;
    cpu.program_counter += 1;

    let address = bus.read_u16(pointer_address);
    let offset_address = address + cpu.y as u16;
    let add_cycle = offset_address & 0xFF00 != address & 0xFF00;

    Box::new(MemoryAddressingMode {
        address: offset_address,
        additional_cycles_required: add_cycle as u8,
    })
};

/// Relative addressing mode
///
/// Only branch instructions use this.
pub(super) const RELATIVE: AddressingModeFactory<i8> = |cpu: &mut Cpu, _: &mut Bus| {
    let address = cpu.program_counter;
    cpu.program_counter += 1;

    Box::new(RelativeAddressingMode {
        address,
        additional_cycles_required: 0,
    })
};
