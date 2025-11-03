use std::{fmt::Debug, sync::LazyLock};

use crate::hardware::{
    bus::Bus,
    cpu::{Cpu, addressing_modes::*, operations::*},
};

pub(super) struct InstructionFactory<T> {
    operation: Operation<T>,
    operation_name: &'static str,
    addressing_mode_factory: AddressingModeFactory<T>,
    cycles: u8,
}

pub(super) trait InstructionFactoryTrait: Send + Sync {
    /// # Returns:
    /// An executable and dissassemblable instruction
    fn create(&self, cpu: &mut Cpu, bus: &mut Bus) -> Box<dyn InstructionTrait>;
}

impl<T: 'static + Debug> InstructionFactoryTrait for InstructionFactory<T> {
    fn create(&self, cpu: &mut Cpu, bus: &mut Bus) -> Box<dyn InstructionTrait> {
        let addressing_mode = (self.addressing_mode_factory)(cpu, bus);
        Box::new(Instruction {
            operation: self.operation,
            addressing_mode,
            cycles: self.cycles,
            operation_name: self.operation_name,
        })
    }
}

pub(super) struct Instruction<T> {
    operation: Operation<T>,
    operation_name: &'static str,
    addressing_mode: Box<dyn AddressingMode<T>>,
    cycles: u8,
}

pub(super) trait InstructionTrait {
    /// # Returns:
    /// The ammount of cycles required for that instruction to be executed
    fn execute(&mut self, cpu: &mut Cpu, bus: &mut Bus) -> u8;
    /// # Returns:
    /// The dissassembled version of the instruction in string slice
    fn dissassemble_instruction(&self, cpu: &mut Cpu, bus: &mut Bus) -> String;
}

impl<T: Debug> InstructionTrait for Instruction<T> {
    fn execute(&mut self, cpu: &mut Cpu, bus: &mut Bus) -> u8 {
        (self.operation)(cpu, bus, &mut self.addressing_mode);
        return self.cycles + self.addressing_mode.additional_cycles_required();
    }
    fn dissassemble_instruction(&self, cpu: &mut Cpu, bus: &mut Bus) -> String {
        format!(
            "{} {:#?}",
            self.operation_name,
            self.addressing_mode.read(cpu, bus)
        )
    }
}

fn instruction_factory<T: 'static + Debug>(
    operation: Operation<T>,
    mode: AddressingModeFactory<T>,
    cycles: u8,
    name: &'static str,
) -> Box<dyn InstructionFactoryTrait> {
    Box::new(InstructionFactory {
        operation: operation,
        addressing_mode_factory: mode,
        cycles: cycles,
        operation_name: name,
    })
}

macro_rules! instruction_factories {
    ($({ $instruction:ident, $mode:ident, $cycles:literal }),* $(,)?) => {
        vec![$(instruction_factory($instruction, $mode, $cycles, stringify!($instruction))),*]
    };
}

pub(super) static INSTRUCTIONS_LOOKUP: LazyLock<&'static [Box<dyn InstructionFactoryTrait>]> =
    LazyLock::new(|| {
        let ops_slice = get_instructions().into_boxed_slice();
        Box::leak(ops_slice)
    });

#[rustfmt::skip]
fn get_instructions() -> Vec<Box<dyn InstructionFactoryTrait>> {
    instruction_factories![
        { BRK, IMPLICIT        , 7 }, { ORA, INDIRECT_X_OFFSET, 6 }, { ILL, IMPLICIT,  2 }, { ILL, IMPLICIT, 8 }, { NOP, IMPLICIT          , 3 }, { ORA, ZERO_PAGE         , 3 },{ ASL, ZERO_PAGE         , 5 }, { ILL, IMPLICIT, 5 }, { PHP, IMPLICIT, 3 }, { ORA, IMMEDIATE        , 2 }, { ASL, ACCUMULATOR, 2 }, { ILL, IMPLICIT   , 2 }, { NOP, IMPLICIT         , 4 }, { ORA, ABSOLUTE         , 4 }, { ASL, ABSOLUTE         , 6 }, { ILL, IMPLICIT, 6 },
        { BPL, RELATIVE        , 2 }, { ORA, INDIRECT_Y_OFFSET, 5 }, { ILL, IMPLICIT,  2 }, { ILL, IMPLICIT, 8 }, { NOP, IMPLICIT          , 4 }, { ORA, ZERO_PAGE_X_OFFSET, 4 },{ ASL, ZERO_PAGE_X_OFFSET, 6 }, { ILL, IMPLICIT, 6 }, { CLC, IMPLICIT, 2 }, { ORA, ABSOLUTE_Y_OFFSET, 4 }, { NOP, IMPLICIT   , 2 }, { ILL, IMPLICIT   , 7 }, { NOP, IMPLICIT         , 4 }, { ORA, ABSOLUTE_X_OFFSET, 4 }, { ASL, ABSOLUTE_X_OFFSET, 7 }, { ILL, IMPLICIT, 7 },
        { JSR, ABSOLUTE_JUMPING, 6 }, { AND, INDIRECT_X_OFFSET, 6 }, { ILL, IMPLICIT,  2 }, { ILL, IMPLICIT, 8 }, { BIT, ZERO_PAGE         , 3 }, { AND, ZERO_PAGE         , 3 },{ ROL, ZERO_PAGE         , 5 }, { ILL, IMPLICIT, 5 }, { PLP, IMPLICIT, 4 }, { AND, IMMEDIATE        , 2 }, { ROL, ACCUMULATOR, 2 }, { ILL, IMPLICIT   , 2 }, { BIT, ABSOLUTE         , 4 }, { AND, ABSOLUTE         , 4 }, { ROL, ABSOLUTE         , 6 }, { ILL, IMPLICIT, 6 },
        { BMI, RELATIVE        , 2 }, { AND, INDIRECT_Y_OFFSET, 5 }, { ILL, IMPLICIT,  2 }, { ILL, IMPLICIT, 8 }, { NOP, IMPLICIT          , 4 }, { AND, ZERO_PAGE_X_OFFSET, 4 },{ ROL, ZERO_PAGE_X_OFFSET, 6 }, { ILL, IMPLICIT, 6 }, { SEC, IMPLICIT, 2 }, { AND, ABSOLUTE_Y_OFFSET, 4 }, { NOP, IMPLICIT   , 2 }, { ILL, IMPLICIT   , 7 }, { NOP, IMPLICIT         , 4 }, { AND, ABSOLUTE_X_OFFSET, 4 }, { ROL, ABSOLUTE_X_OFFSET, 7 }, { ILL, IMPLICIT, 7 },
        { RTI, IMPLICIT        , 6 }, { EOR, INDIRECT_X_OFFSET, 6 }, { ILL, IMPLICIT,  2 }, { ILL, IMPLICIT, 8 }, { NOP, IMPLICIT          , 3 }, { EOR, ZERO_PAGE         , 3 },{ LSR, ZERO_PAGE         , 5 }, { ILL, IMPLICIT, 5 }, { PHA, IMPLICIT, 3 }, { EOR, IMMEDIATE        , 2 }, { LSR, ACCUMULATOR, 2 }, { ILL, IMPLICIT   , 2 }, { JMP, ABSOLUTE_JUMPING , 3 }, { EOR, ABSOLUTE         , 4 }, { LSR, ABSOLUTE         , 6 }, { ILL, IMPLICIT, 6 },
        { BVC, RELATIVE        , 2 }, { EOR, INDIRECT_Y_OFFSET, 5 }, { ILL, IMPLICIT,  2 }, { ILL, IMPLICIT, 8 }, { NOP, IMPLICIT          , 4 }, { EOR, ZERO_PAGE_X_OFFSET, 4 },{ LSR, ZERO_PAGE_X_OFFSET, 6 }, { ILL, IMPLICIT, 6 }, { CLI, IMPLICIT, 2 }, { EOR, ABSOLUTE_Y_OFFSET, 4 }, { NOP, IMPLICIT   , 2 }, { ILL, IMPLICIT   , 7 }, { NOP, IMPLICIT         , 4 }, { EOR, ABSOLUTE_X_OFFSET, 4 }, { LSR, ABSOLUTE_X_OFFSET, 7 }, { ILL, IMPLICIT, 7 },
        { RTS, IMPLICIT        , 6 }, { ADC, INDIRECT_X_OFFSET, 6 }, { ILL, IMPLICIT,  2 }, { ILL, IMPLICIT, 8 }, { NOP, IMPLICIT          , 3 }, { ADC, ZERO_PAGE         , 3 },{ ROR, ZERO_PAGE         , 5 }, { ILL, IMPLICIT, 5 }, { PLA, IMPLICIT, 4 }, { ADC, IMMEDIATE        , 2 }, { ROR, ACCUMULATOR, 2 }, { ILL, IMPLICIT   , 2 }, { JMP, INDIRECT_JUMPING , 5 }, { ADC, ABSOLUTE         , 4 }, { ROR, ABSOLUTE         , 6 }, { ILL, IMPLICIT, 6 },
        { BVS, RELATIVE        , 2 }, { ADC, INDIRECT_Y_OFFSET, 5 }, { ILL, IMPLICIT,  2 }, { ILL, IMPLICIT, 8 }, { NOP, IMPLICIT          , 4 }, { ADC, ZERO_PAGE_X_OFFSET, 4 },{ ROR, ZERO_PAGE_X_OFFSET, 6 }, { ILL, IMPLICIT, 6 }, { SEI, IMPLICIT, 2 }, { ADC, ABSOLUTE_Y_OFFSET, 4 }, { NOP, IMPLICIT   , 2 }, { ILL, IMPLICIT   , 7 }, { NOP, IMPLICIT         , 4 }, { ADC, ABSOLUTE_X_OFFSET, 4 }, { ROR, ABSOLUTE_X_OFFSET, 7 }, { ILL, IMPLICIT, 7 },
        { NOP, IMPLICIT        , 2 }, { STA, INDIRECT_X_OFFSET, 6 }, { NOP, IMPLICIT,  2 }, { ILL, IMPLICIT, 6 }, { STY, ZERO_PAGE         , 3 }, { STA, ZERO_PAGE         , 3 },{ STX, ZERO_PAGE         , 3 }, { ILL, IMPLICIT, 3 }, { DEY, IMPLICIT, 2 }, { NOP, IMPLICIT         , 2 }, { TXA, IMPLICIT   , 2 }, { ILL, IMPLICIT   , 2 }, { STY, ABSOLUTE         , 4 }, { STA, ABSOLUTE         , 4 }, { STX, ABSOLUTE         , 4 }, { ILL, IMPLICIT, 4 },
        { BCC, RELATIVE        , 2 }, { STA, INDIRECT_Y_OFFSET, 6 }, { ILL, IMPLICIT,  2 }, { ILL, IMPLICIT, 6 }, { STY, ZERO_PAGE_X_OFFSET, 4 }, { STA, ZERO_PAGE_X_OFFSET, 4 },{ STX, ZERO_PAGE_Y_OFFSET, 4 }, { ILL, IMPLICIT, 4 }, { TYA, IMPLICIT, 2 }, { STA, ABSOLUTE_Y_OFFSET, 5 }, { TXS, IMPLICIT   , 2 }, { ILL, IMPLICIT   , 5 }, { NOP, IMPLICIT         , 5 }, { STA, ABSOLUTE_X_OFFSET, 5 }, { ILL, IMPLICIT         , 5 }, { ILL, IMPLICIT, 5 },
        { LDY, IMMEDIATE       , 2 }, { LDA, INDIRECT_X_OFFSET, 6 }, { LDX, IMMEDIATE, 2 }, { ILL, IMPLICIT, 6 }, { LDY, ZERO_PAGE         , 3 }, { LDA, ZERO_PAGE         , 3 },{ LDX, ZERO_PAGE         , 3 }, { ILL, IMPLICIT, 3 }, { TAY, IMPLICIT, 2 }, { LDA, IMMEDIATE        , 2 }, { TAX, IMPLICIT   , 2 }, { ILL, IMPLICIT   , 2 }, { LDY, ABSOLUTE         , 4 }, { LDA, ABSOLUTE         , 4 }, { LDX, ABSOLUTE         , 4 }, { ILL, IMPLICIT, 4 },
        { BCS, RELATIVE        , 2 }, { LDA, INDIRECT_Y_OFFSET, 5 }, { ILL, IMPLICIT,  2 }, { ILL, IMPLICIT, 5 }, { LDY, ZERO_PAGE_X_OFFSET, 4 }, { LDA, ZERO_PAGE_X_OFFSET, 4 },{ LDX, ZERO_PAGE_Y_OFFSET, 4 }, { ILL, IMPLICIT, 4 }, { CLV, IMPLICIT, 2 }, { LDA, ABSOLUTE_Y_OFFSET, 4 }, { TSX, IMPLICIT   , 2 }, { ILL, IMPLICIT   , 4 }, { LDY, ABSOLUTE_X_OFFSET, 4 }, { LDA, ABSOLUTE_X_OFFSET, 4 }, { LDX, ABSOLUTE_Y_OFFSET, 4 }, { ILL, IMPLICIT, 4 },
        { CPY, IMMEDIATE       , 2 }, { CMP, INDIRECT_X_OFFSET, 6 }, { NOP, IMPLICIT,  2 }, { ILL, IMPLICIT, 8 }, { CPY, ZERO_PAGE         , 3 }, { CMP, ZERO_PAGE         , 3 },{ DEC, ZERO_PAGE         , 5 }, { ILL, IMPLICIT, 5 }, { INY, IMPLICIT, 2 }, { CMP, IMMEDIATE        , 2 }, { DEX, IMPLICIT   , 2 }, { ILL, IMPLICIT   , 2 }, { CPY, ABSOLUTE         , 4 }, { CMP, ABSOLUTE         , 4 }, { DEC, ABSOLUTE         , 6 }, { ILL, IMPLICIT, 6 },
        { BNE, RELATIVE        , 2 }, { CMP, INDIRECT_Y_OFFSET, 5 }, { ILL, IMPLICIT,  2 }, { ILL, IMPLICIT, 8 }, { NOP, IMPLICIT          , 4 }, { CMP, ZERO_PAGE_X_OFFSET, 4 },{ DEC, ZERO_PAGE_X_OFFSET, 6 }, { ILL, IMPLICIT, 6 }, { CLD, IMPLICIT, 2 }, { CMP, ABSOLUTE_Y_OFFSET, 4 }, { NOP, IMPLICIT   , 2 }, { ILL, IMPLICIT   , 7 }, { NOP, IMPLICIT         , 4 }, { CMP, ABSOLUTE_X_OFFSET, 4 }, { DEC, ABSOLUTE_X_OFFSET, 7 }, { ILL, IMPLICIT, 7 },
        { CPX, IMMEDIATE       , 2 }, { SBC, INDIRECT_X_OFFSET, 6 }, { NOP, IMPLICIT,  2 }, { ILL, IMPLICIT, 8 }, { CPX, ZERO_PAGE         , 3 }, { SBC, ZERO_PAGE         , 3 },{ INC, ZERO_PAGE         , 5 }, { ILL, IMPLICIT, 5 }, { INX, IMPLICIT, 2 }, { SBC, IMMEDIATE        , 2 }, { NOP, IMPLICIT   , 2 }, { SBC, ACCUMULATOR, 2 }, { CPX, ABSOLUTE         , 4 }, { SBC, ABSOLUTE         , 4 }, { INC, ABSOLUTE         , 6 }, { ILL, IMPLICIT, 6 },
        { BEQ, RELATIVE        , 2 }, { SBC, INDIRECT_Y_OFFSET, 5 }, { ILL, IMPLICIT,  2 }, { ILL, IMPLICIT, 8 }, { NOP, IMPLICIT          , 4 }, { SBC, ZERO_PAGE_X_OFFSET, 4 },{ INC, ZERO_PAGE_X_OFFSET, 6 }, { ILL, IMPLICIT, 6 }, { SED, IMPLICIT, 2 }, { SBC, ABSOLUTE_Y_OFFSET, 4 }, { NOP, IMPLICIT   , 2 }, { ILL, IMPLICIT   , 7 }, { NOP, IMPLICIT         , 4 }, { SBC, ABSOLUTE_X_OFFSET, 4 }, { INC, ABSOLUTE_X_OFFSET, 7 }, { ILL, IMPLICIT, 7 },
    ]
}
