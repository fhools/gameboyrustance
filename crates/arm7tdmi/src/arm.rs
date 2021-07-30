use super::InstructionType;

pub struct Instruction(u32);

impl Instruction {
    fn new(i: u32) -> Self {
        Instruction(i)
    }

    fn instr_type(&self) -> InstructionType {
        // Bits 27-26
        // 0b10 = Branch
        // 0b00 = Data Processing (ALI)
        // 0b01 = Memory Instruction.
    }
}

pub struct ARMCpu {
}
