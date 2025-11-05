#![cfg(test)]

use crate::disassembler::Dissasembler;

fn decomp_test(program: &str, memory: &[u8]) {
    let mut disassembler = Dissasembler::new(0x8000, memory);

    let disassembled_program = disassembler.disassemble();

    assert_eq!(program, disassembled_program);
}

#[test]
fn fibbo() {
    decomp_test(
        "LDA #$00
STA $00
LDA #$01
STA $01
LDX #$00
LDA $00,x
CLC 
ADC $01,x
STA $02,x
INX 
BCC *-$08
INX",
        &[
            0xA9, 0x00, 0x85, 0x00, 0xA9, 0x01, 0x85, 0x01, 0xA2, 0x00, 0xB5, 0x00, 0x18, 0x75,
            0x01, 0x95, 0x02, 0xE8, 0x90, 0xF6, 0xE8,
        ],
    );
}
