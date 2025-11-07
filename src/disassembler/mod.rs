mod test;
use crate::hardware::{bus::Bus, cpu::Cpu};

pub struct Dissasembler {
    cpu: Cpu,
    bus: Bus,
    end: u16,
}

/// TODO: make dissasembling actually find the high entropy regions and
/// only dissasemble those. Remove the behaviour where the disassembling
/// process stops once the disassembler reaches a 0x00 (BRK)
impl Dissasembler {
    pub fn new(start: u16, memory: &[u8]) -> Self {
        let mut cpu = Cpu::new();
        let mut bus = Bus::new();

        bus.write_u16(0xFFFC, start);
        cpu.reset(&bus);
        bus.write_memory(start, memory);

        Self {
            cpu: cpu,
            bus: bus,
            end: start + memory.len() as u16,
        }
    }

    pub fn disassemble(&mut self) -> String {
        let mut output = String::new();

        loop {
            let instruction = self.cpu.get_next_instruction(&self.bus);
            output += instruction.disassemble_instruction().as_str();
            output += "\n";
            if self.cpu.get_program_counter() >= self.end {
                break;
            }
        }

        output.trim().to_string()
    }
}
