
use util::get_bits;
use super::InstructionType;
use std::io::ErrorKind;
use std::io;
#[derive(Debug)]
pub struct Instruction(u32);

impl Instruction {
    pub fn new(i: u32) -> Self {
        Instruction(i)
    }

    pub fn instr_type(&self) -> Result<InstructionType, std::io::Error> {
        // Bits 27-26
        // 0b10 = Branch
        // 0b00 = Data Processing (ALI)
        // 0b01 = Memory Instruction.
        let bits27_26 = get_bits(self.0, 26, 27);
        if bits27_26 == 0b01 {
            return Ok(InstructionType::MemoryProcessing)
        }

        if bits27_26  == 0b10 {
            return Ok(InstructionType::Branch)
        }

        if bits27_26 == 0b00 {
            return Ok(InstructionType::DataProcessing)
        }
        return Err(io::Error::new(io::ErrorKind::InvalidData, "invalid instruction bits 27-26"));
    }
}

pub struct ARMCpu {
}

mod tests {
    use super::*;
    #[test]
    fn test_instruction() {
        let mut i = Instruction::new(0x0);
        assert_eq!(i.instr_type().unwrap(), InstructionType::DataProcessing);
        i = Instruction::new(0xea00_002e);
        assert_eq!(i.instr_type().unwrap(), InstructionType::Branch);
    }
}
