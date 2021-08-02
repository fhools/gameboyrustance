use util::get_bits;
use super::InstructionType;
use super::ConditionField;
use std::io::ErrorKind;
use std::io;
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

        if bits27_26  == 0b10 {
            return InstructionType::Branch
        }

        if bits27_26 == 0b00 {
            return InstructionType::DataProcessing
        }

        panic!("unknown instruction type {:b}", bits27_26)
    }

    pub fn cond(&self) -> ConditionField {
        let bits31_28 = get_bits(self.0, 28, 31);
        ConditionField::new(bits31_28 as u8)
    }

    pub fn operand2_mode(&self) -> bool {
        let bit25 = get_bits(self.0, 25, 25);
        bit25 == 1
    }

    pub fn operand2(&self) -> u32 {
        let bits11_0 = get_bits(self.0, 0, 11);
        bits11_0
    }
}


pub struct DataProcessing {
}

impl DataProcessing {
    const OPCODE_NAMES : [&'static str;16] = [
        "AND", // 0b0000
        "EOR",
        "SUB",
        "RSB",
        "ADD",
        "ADC",
        "SBC",
        "RSC",
        "TST",
        "TEQ",
        "CMP",
        "CMN",
        "ORR",
        "MOV",
        "BIC",
        "MVN" // 0b1111
    ];

}
pub struct ARMCpu {
}

mod tests {
    use super::*;
    #[test]
    fn test_instruction() {
        let mut i = Instruction::new(0x0);
        assert_eq!(i.instr_type(), InstructionType::DataProcessing);
        i = Instruction::new(0xea00_002e);
        assert_eq!(i.instr_type(), InstructionType::Branch);
        i = Instruction::new(0xe3a00301);
        assert_eq!(i.instr_type(), InstructionType::DataProcessing);
    }

    #[test]
    fn test_conditionfields() {
        let mut i = Instruction::new(0x159f_0018); // instr: ldrne
        let condf = i.cond();
        println!("condf: {}, condf hex: {:x}", condf, condf.0);
        assert_eq!(condf.0, ConditionField::COND_NE);
    }

}
