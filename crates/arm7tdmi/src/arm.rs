use super::ConditionField;
use super::InstructionType;
use std::io;
use std::io::ErrorKind;
use util::get_bits;

#[derive(Debug, PartialEq)]
pub enum ArmV4Type {
    Multiply,
    MultiplyAccum,
    BranchAndExchange,
    SingleDataSwap,
    HalfwordDataTransferReg,
    HalfwordDataTransferImm,
    SignedDataTransfer,
    DataProcessingPsr,
    LoadStore,
    Undefined,
    BlockDataTransfer,
    Branch,
    CoprocDataTransfer,
    CoprocDataOp,
    CoprocRegTransfer,
    SoftwareInterrupt,
}

pub fn armv4_type(i: u32) -> ArmV4Type {
    let bits27_22 = get_bits(i, 22, 27);
    let bits7_4 = get_bits(i, 4, 7);
    if bits27_22 == 0 && bits7_4 == 0b1001 {
        return ArmV4Type::Multiply;
    }

    let bits27_23 = get_bits(i, 23, 27);
    if bits27_23 == 1 && bits7_4 == 0b1001 {
        return ArmV4Type::MultiplyAccum;
    }

    let bits27_4 = get_bits(i, 4, 27);
    if bits27_4 == 0b0001_0010_1111_1111_1111_0001 {
        return ArmV4Type::BranchAndExchange;
    }

    let bits11_4 = get_bits(i, 4, 11);
    let bits21_20 = get_bits(i, 20, 21);
    if bits27_23 == 0b00010 && bits21_20 == 0 && bits11_4 == 0b0000_1001 {
        return ArmV4Type::SingleDataSwap;
    }

    let bits27_25 = get_bits(i, 25, 27);
    let bit22 = get_bits(i, 22, 22);
    if bits27_25 == 0 && bit22 == 0 && bits11_4 == 0b0000_1011 {
        return ArmV4Type::HalfwordDataTransferReg;
    }

    if bits27_25 == 0 && bit22 == 1 && bits7_4 == 0b1011 {
        return ArmV4Type::HalfwordDataTransferImm;
    }

    let bits7_6 = get_bits(i, 6, 7);
    let bit4 = get_bits(i, 4, 4);
    if bits27_25 == 0 && bits7_6 == 0b11 && bit4 == 1 {
        return ArmV4Type::SignedDataTransfer;
    }

    let bits27_26 = get_bits(i, 26, 27);
    if bits27_26 == 0 {
        return ArmV4Type::DataProcessingPsr;
    }

    if bits27_26 == 0b01 {
        return ArmV4Type::LoadStore;
    }

    if bits27_25 == 0b011 && bit4 == 1 {
        return ArmV4Type::Undefined;
    }

    if bits27_25 == 0b100 && bit22 == 0 {
        return ArmV4Type::BlockDataTransfer;
    }

    if bits27_25 == 0b101 {
        return ArmV4Type::Branch;
    }

    if bits27_25 == 0b110 {
        return ArmV4Type::CoprocDataTransfer;
    }

    let bits27_24 = get_bits(i, 24, 27);
    if bits27_24 == 0b1110 && bit4 == 0 {
        return ArmV4Type::CoprocDataOp;
    }

    if bits27_24 == 0b1110 && bit4 == 1 {
        return ArmV4Type::CoprocRegTransfer;
    }

    if bits27_24 == 0b1111 {
        return ArmV4Type::SoftwareInterrupt;
    }

    panic!("Unknown instruction: 0x{:x}", i);
}
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
            return InstructionType::MemoryProcessing;
        }

        if bits27_26 == 0b10 {
            return InstructionType::Branch;
        }

        if bits27_26 == 0b00 {
            return InstructionType::DataProcessing;
        }

        panic!("unknown instruction type {:b}", bits27_26)
    }

    pub fn cond(&self) -> ConditionField {
        let bits31_28 = get_bits(self.0, 28, 31);
        ConditionField::new(bits31_28 as u8)
    }

    pub fn is_operand2_imm(&self) -> bool {
        let bit25 = get_bits(self.0, 25, 25);
        bit25 == 1
    }

    pub fn dataprocessing_opcode_str(&self) -> &'static str {
        let opcode = get_bits(self.0, 21, 24);
        DataProcessing::OPCODE_NAMES[opcode as usize]
    }

    pub fn s_bit(&self) -> bool {
        let bit20 = get_bits(self.0, 20, 20);
        bit20 == 1
    }

    pub fn dataprocessing_rn(&self) -> usize {
        let bits19_16 = get_bits(self.0, 16, 19);
        bits19_16 as usize
    }

    pub fn dataprocessing_rd(&self) -> usize {
        let bits15_12 = get_bits(self.0, 12, 15);
        bits15_12 as usize
    }

    pub fn data_processing_operand2_as_shiftreg(&self) -> (usize, usize) {
        let shift = get_bits(self.0, 4, 11);
        let rm = get_bits(self.0, 0, 3);
        (shift as usize, rm as usize)
    }

    pub fn dataprocessing_operand2_as_rotimm(&self) -> (usize, usize) {
        let rot = get_bits(self.0, 8, 11);
        let imm = get_bits(self.0, 0, 7);
        return (rot as usize, imm as usize);
    }

    pub fn dataprocessing_operand2(&self) -> u32 {
        let bits11_0 = get_bits(self.0, 0, 11);
        bits11_0
    }
}

pub struct DataProcessing {}
#[derive(Debug)]
pub enum DataProcessingOpCode {
    And = 0,
    Eor = 1,
    Sub = 2,
    Rsb = 3,
    Add = 4,
    Adc = 5,
    Sbc = 6,
    Rsc = 7,
    Tst = 8,
    Teq = 9,
    Cmp = 10,
    Cmn = 11,
    Orr = 12,
    Mov = 13,
    Bic = 14,
    Mvn = 15,
}

#[derive(Debug)]
pub enum ShiftType {
    LogicalLeft = 0b00,
    LogicalRight = 0b01,
    ArithmeticLeft = 0b10,
    ArithemticRight = 0b11,
}

#[derive(Debug)]
pub enum DataProcessingOperand2 {
    // bits 11-7 = shift amount, bits 6-5 = shift type , bit 4 = 0
    ShiftRegDirect {
        shift_count: usize,
        shift_type: ShiftType,
    },
    // bits 11-7 = register #, bits 7 = 0, bits 6-5 = shift type, bit 4 = 1
    ShiftRegIndirect {
        shift_reg: usize,
        shift_type: ShiftType,
    },
    // bits 11-8 = rotate amount, bits 7-0 = immediate value
    // shift ammount is rotate_amount * 2
    ImmRot {
        rotate_count: usize,
        imm_value: usize,
    },
}

impl DataProcessing {
    const OPCODE_NAMES: [&'static str; 16] = [
        "AND", // 0b0000
        "EOR", "SUB", "RSB", "ADD", "ADC", "SBC", "RSC", "TST", "TEQ", "CMP", "CMN", "ORR", "MOV",
        "BIC", "MVN", // 0b1111
    ];
}
pub struct ARMCpu {}

mod tests {
    use super::*;
    use util::read_instructions_file;
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
        let i = Instruction::new(0x159f_0018); // instr: ldrne
        let condf = i.cond();
        println!("condf: {}, condf hex: {:x}", condf, condf.0);
        assert_eq!(condf.0, ConditionField::COND_NE);
    }

    #[test]
    fn test_dataprocessing_instruction() {
        // e3a00301 mov r0, #0x4000.0000
        let i = Instruction::new(0xe3a00301);
        println!("op_code: {}", i.dataprocessing_opcode_str());
        assert_eq!(ArmV4Type::DataProcessingPsr, armv4_type(i.0));
    }

    #[test]
    fn test_dataprocessing_instrutions2() {
        let v = read_instructions_file("a.gba", 0xe0, 10).unwrap();
        let e = v
            .iter()
            .map::<ArmV4Type, fn(&u32) -> ArmV4Type>(|&i| armv4_type(i))
            .collect::<Vec<ArmV4Type>>();
        println!("e: {:?}", e);
    }
}
