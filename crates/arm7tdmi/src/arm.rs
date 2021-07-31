
use util::get_bits;
use super::InstructionType;

#[derive(Debug)]
pub struct Instruction(u32);

impl Instruction {
    pub fn new(i: u32) -> Self {
        Instruction(i)
    }

    pub fn instr_type(&self) -> InstructionType {
        // Bits 27-26
        // 0b10 = Branch
        // 0b00 = Data Processing (ALI)
        // 0b01 = Memory Instruction.
        let bits27_26 = get_bits(self.0, 26, 27);
        if bits27_26 == 0b01 {
            return InstructionType::MemoryProcessing
        }
        InstructionType::Branch
    }
}

pub struct ARMCpu {
}

mod tests {
    use super::*;
    #[test]
    fn test_instruction() {
        let i = Instruction::new(0x0);
        assert_eq!(i.instr_type(), InstructionType::Branch);
    }
}
