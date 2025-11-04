//! # Factories
//!
//! This module is responsible for creating simple helpers that return
//! an [super::AddressingMode]. They are used in the
//! [lookup table](crate::hardware::cpu::instructions::).
use crate::hardware::{
    bus::Bus,
    cpu::{Cpu, addressing_modes::AddressingMode},
};

use super::implementations::*;

fn format_hex_u8(value: u8) -> String {
    format!("${value:02x}")
}

fn format_hex_i8(value: i8) -> String {
    if value < 0 {
        format!("-${:02x}", (-value) as u8)
    } else {
        format!("${:02x}", value as u8)
    }
}

fn format_hex_u16(value: u16) -> String {
    format!("${value:04x}")
}

pub(crate) type AddressingModeFactory<T> =
    fn(cpu: &mut Cpu, bus: &Bus) -> Box<dyn AddressingMode<T>>;

// /// Implicit addressing mode
// ///
// /// Instructions using implicit mode do not require a parameter (ex: CLC)
pub(crate) const IMPLICIT: AddressingModeFactory<()> = |_: &mut Cpu, _: &Bus| {
    Box::new(ImplicitAddressingMode {
        additional_cycles_required: 0,
    })
};

/// Accumulator addressing mode
///
/// Gets the acculumator as the argument
// pub(crate) const ACCUMULATOR: AddressingMode<&mut u8> =
//     |cpu: &mut Cpu, _: &mut Bus| -> CycleEffect<&mut u8> {};
pub(crate) const ACCUMULATOR: AddressingModeFactory<u8> = |_: &mut Cpu, _: &Bus| {
    Box::new(AccumulatorAddressingMode {
        additional_cycles_required: 0,
        display: format!("A"),
    })
};

/// Immediate addressing mode
///
/// Gets the next byte as the argument
pub(crate) const IMMEDIATE: AddressingModeFactory<u8> = |cpu: &mut Cpu, bus: &Bus| {
    let address = cpu.program_counter;
    cpu.program_counter += 1;

    let value = bus.read(address);

    Box::new(MemoryAddressingMode {
        address,
        additional_cycles_required: 0,
        display: format!("#{}", format_hex_u8(value)),
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
pub(crate) const ZERO_PAGE: AddressingModeFactory<u8> = |cpu: &mut Cpu, bus: &Bus| {
    let address = bus.read(cpu.program_counter) as u16;
    cpu.program_counter += 1;

    Box::new(MemoryAddressingMode {
        address,
        additional_cycles_required: 0,
        display: format!("{}", format_hex_u8(address as u8),),
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
pub(crate) const ZERO_PAGE_X_OFFSET: AddressingModeFactory<u8> = |cpu: &mut Cpu, bus: &Bus| {
    let address = bus.read(cpu.program_counter).wrapping_add(cpu.x) as u16;
    cpu.program_counter += 1;

    Box::new(MemoryAddressingMode {
        address,
        additional_cycles_required: 0,
        display: format!("{},x", format_hex_u8(address as u8)),
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
pub(crate) const ZERO_PAGE_Y_OFFSET: AddressingModeFactory<u8> = |cpu: &mut Cpu, bus: &Bus| {
    let address = bus.read(cpu.program_counter).wrapping_add(cpu.y) as u16;
    cpu.program_counter += 1;

    Box::new(MemoryAddressingMode {
        address,
        additional_cycles_required: 0,
        display: format!("{},y", format_hex_u8(address as u8)),
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
pub(crate) const ABSOLUTE: AddressingModeFactory<u8> = |cpu: &mut Cpu, bus: &Bus| {
    let address = bus.read_u16(cpu.program_counter);
    cpu.program_counter += 2;

    Box::new(MemoryAddressingMode {
        address,
        additional_cycles_required: 0,
        display: format!("{}", format_hex_u16(address)),
    })
};

/// Absolute addressing mode
///
///
/// Used for jump instructions to allow them to also access the memory location
pub(crate) const ABSOLUTE_JUMPING: AddressingModeFactory<JumpAddress> =
    |cpu: &mut Cpu, bus: &Bus| {
        let address = bus.read_u16(cpu.program_counter);
        cpu.program_counter += 2;

        Box::new(JumpingAddressingMode {
            address,
            additional_cycles_required: 0,
            display: format!("{}", format_hex_u16(address)),
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
pub(crate) const ABSOLUTE_X_OFFSET: AddressingModeFactory<u8> = |cpu: &mut Cpu, bus: &Bus| {
    let address = bus.read_u16(cpu.program_counter);
    cpu.program_counter += 2;
    let offset_address = address + cpu.x as u16;

    let add_cycle = offset_address & 0xFF00 != address & 0xFF00;

    Box::new(MemoryAddressingMode {
        address: offset_address,
        additional_cycles_required: add_cycle as u8,
        display: format!("{},x", format_hex_u16(address)),
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
pub(crate) const ABSOLUTE_Y_OFFSET: AddressingModeFactory<u8> = |cpu: &mut Cpu, bus: &Bus| {
    let address = bus.read_u16(cpu.program_counter);
    cpu.program_counter += 2;
    let offset_address = address + cpu.y as u16;

    let add_cycle = offset_address & 0xFF00 != address & 0xFF00;

    Box::new(MemoryAddressingMode {
        address: offset_address,
        additional_cycles_required: add_cycle as u8,
        display: format!("{},y", format_hex_u16(address)),
    })
};

// apparently this is only used for jumping and the INDIRECT_JUMPING already exists so wtv
// /// Indirect addressing mode
// ///
// /// Reads a 16-bit pointer from the next two bytes and returns the
// /// pointed-to address.
// pub(crate) const INDIRECT: AddressingModeFactory<u8> = |cpu: &mut Cpu, bus: &Bus| {
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
pub(crate) const INDIRECT_JUMPING: AddressingModeFactory<JumpAddress> =
    |cpu: &mut Cpu, bus: &Bus| {
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
            display: format!("({})", format_hex_u16(address)),
        })
    };

/// Indirect with x offset addressing mode
///
/// Reads an 8-bit pointer to a zero page location from the next byte + x
/// and then uses that as the actual address.
pub(crate) const INDIRECT_X_OFFSET: AddressingModeFactory<u8> = |cpu: &mut Cpu, bus: &Bus| {
    let pointer_address = bus.read(cpu.program_counter) as u16;
    cpu.program_counter += 1;

    let address = bus.read_u16(pointer_address + cpu.x as u16);

    Box::new(MemoryAddressingMode {
        address,
        additional_cycles_required: 0,
        display: format!("({},x)", format_hex_u16(address)),
    })
};

/// Indirect with y offset addressing mode
///
/// Reads an 8-bit pointer to a zero page location from the next byte
/// and then adds y to that loccation and returns that new address.
pub(crate) const INDIRECT_Y_OFFSET: AddressingModeFactory<u8> = |cpu: &mut Cpu, bus: &Bus| {
    let pointer_address = bus.read(cpu.program_counter) as u16;
    cpu.program_counter += 1;

    let address = bus.read_u16(pointer_address);
    let offset_address = address + cpu.y as u16;
    let add_cycle = offset_address & 0xFF00 != address & 0xFF00;

    Box::new(MemoryAddressingMode {
        address: offset_address,
        additional_cycles_required: add_cycle as u8,
        display: format!("({}),y", format_hex_u16(address)),
    })
};

/// Relative addressing mode
///
/// Only branch instructions use this.
pub(crate) const RELATIVE: AddressingModeFactory<i8> = |cpu: &mut Cpu, bus: &Bus| {
    let address = cpu.program_counter;
    cpu.program_counter += 1;

    let value = bus.read(address) as i8;

    Box::new(RelativeAddressingMode {
        address,
        additional_cycles_required: 0,
        display: format!("*{}", format_hex_i8(value)),
    })
};
