use std::fmt;
use std::fmt::Display;
pub mod thumb;
pub mod arm;

// ARM7TDMI is an ARM cpu with 2 modes of instruction, a 32-bit ARM and a 16-bit THUMB.
//
// The CPU switches between the two states with a BX instruction.
// The CPU modes share the same register set
// ARM is 32-bit opcodes and THUMB is 16-bit opcodes
//
// ARM7TDMI is a 3 stage pipelined architecture, with fetch, decode, execute stages.
//
//
#[derive(Debug, PartialEq)]
pub enum InstructionType {
    DataProcessing,
    Branch,
    MemoryProcessing
}


/*
 * ConditionField holds bits 31-28 of instructions,
 */
#[derive(Debug, PartialEq)]
pub struct ConditionField(u8);

impl ConditionField {
    // Z = Zero Flag, N = Negative, C = Carry, V = Overflow 
    const COND_EQ : u8 = 0b0000;         // Z ==1
    const COND_NE : u8 = 0b0001;        // Z==0
    const COND_CS_HS : u8 = 0b0010;     // C == 1
    const COND_CC_LO : u8 = 0b0011;     // C == 0
    const COND_MI : u8 = 0b0100;        // N == 1
    const COND_PL : u8 = 0b0101;        // N == 0
    const COND_VS : u8 = 0b0110;        // V == 1
    const COND_VC : u8 = 0b0111;        // V == 0
    const COND_HI : u8 = 0b1000;        // (C == 1) and (Z == 0)
    const COND_LS : u8 = 0b1001;        // (C == 0) or ( Z == 1)
    const COND_GE : u8 = 0b1010;        // N == V 
    const COND_LT : u8 = 0b1011;        // N != V
    const COND_GT : u8 = 0b1100;        // (Z == 0) and (N == V)
    const COND_LE : u8 = 0b1101;        // (Z == 1) or (N != V)
    const COND_AL : u8 = 0b1110;        // Always
    const COND_NV : u8 = 0b1111;        // Should never happen

    fn new(i: u8) -> ConditionField {
        ConditionField(i)
    }

    fn to_str(&self) -> &'static str {
        match self.0 {
            ConditionField::COND_EQ => "EQ",
            ConditionField::COND_NE => "NE",
            ConditionField::COND_CS_HS => "CS_HS",
            ConditionField::COND_CC_LO => "CC_LO",
            ConditionField::COND_MI => "MI",
            ConditionField::COND_PL => "PL",
            ConditionField::COND_VS => "VS",
            ConditionField::COND_VC => "VC",
            ConditionField::COND_HI => "HI",
            ConditionField::COND_LS => "LS",
            ConditionField::COND_GE => "GE",
            ConditionField::COND_LT => "LT",
            ConditionField::COND_GT => "GT",
            ConditionField::COND_LE => "LE",
            ConditionField::COND_AL => "AL",
            ConditionField::COND_NV => "NV",
            _ => panic!("unknown cond_code")
        }
    }
}

impl Display for ConditionField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

