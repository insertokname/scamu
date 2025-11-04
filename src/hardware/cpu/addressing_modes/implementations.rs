//! # Implementations
//!
//! The implementations of different types of
//! [Addressing modes](super::AddressingMode).
use crate::hardware::{bus::Bus, cpu::Cpu};
use std::fmt::Debug;

use super::AddressingMode;

pub(super) struct ImplicitAddressingMode {
    pub(super) additional_cycles_required: u8,
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

    fn display(&self) -> &str {
        ""
    }
}

pub(super) struct AccumulatorAddressingMode {
    pub(super) additional_cycles_required: u8,
    pub(super) display: String,
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

    fn display(&self) -> &str {
        &self.display
    }
}

pub(super) struct MemoryAddressingMode {
    pub(super) address: u16,
    pub(super) additional_cycles_required: u8,
    pub(super) display: String,
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

    fn display(&self) -> &str {
        &self.display
    }
}

pub(super) struct RelativeAddressingMode {
    pub(super) address: u16,
    pub(super) additional_cycles_required: u8,
    pub(super) display: String,
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

    fn display(&self) -> &str {
        &self.display
    }
}

/// Gives the user access to both the address and the value at the address
#[derive(Clone, Copy)]
pub(crate) struct JumpAddress {
    pub(super) value: u8,
    pub(super) address: u16,
}

impl Debug for JumpAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[allow(dead_code)]
impl JumpAddress {
    pub fn get_value(&self) -> &u8 {
        &self.value
    }

    pub fn set_value(&mut self, new_value: u8) {
        self.value = new_value
    }

    pub fn get_address(&self) -> u16 {
        self.address
    }
}

pub(super) struct JumpingAddressingMode {
    pub(super) address: u16,
    pub(super) additional_cycles_required: u8,
    pub(super) display: String,
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

    fn display(&self) -> &str {
        &self.display
    }
}
