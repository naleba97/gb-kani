use crate::cpu::Register8Bit;
use crate::cpu::Register16Bit;
use crate::cpu::ConditionCode;
use std::fmt;
use std::time::Instant;

#[derive(Debug)]
pub enum Opcode{
    NOP,
    STOP,
    INC,
    DEC,
    RLCA,
    RLA,
    DAA,
    SCF,
    RRCA,
    RRA,
    JR,
    LD,
    LDH,
    ADD,
    ADC,
    SUB,
    SBC,
    AND,
    XOR,
    OR,
    CP,
    RET,
    POP,
    PUSH,
    CALL,
    RST,
    JP,
    RLC,
    RRC,
    RL,
    RR,
    SLA,
    SRA,
    BIT,
    RES,
    SET
}

#[derive(Debug)]
struct Operand{
   register_8bit: Option<Register8Bit>,
   register_16bit: Option<Register16Bit>,
   data_8bit: Option<u8>,
   data_16bit: Option<u16>,
   addr_8bit: Option<u8>,
   addr_16bit: Option<u16>,
   pc_relative_8bit: Option<i8>,
   condition_code: Option<ConditionCode>,
}

impl Operand{
    pub fn new() -> Self{
        Operand{
            register_8bit: Some(Register8Bit::A),
            register_16bit: Some(Register16Bit::AF),
            data_8bit: Some(0x0),
            data_16bit: Some(0x0),
            addr_8bit: Some(0x0),
            addr_16bit: Some(0x0),
            pc_relative_8bit: Some(0x0),
            condition_code: Some(ConditionCode::N),
        }
    }
}

#[derive(Default)]
pub struct Instruction {
    binary_value: u8,
    opcode: Opcode,
    operand1: Option<Operand>,
    operand2: Option<Operand>,
}

impl fmt::Display for Operand{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?}", //TODO: comma?
        self.register_8bit.as_ref().unwrap(),
        self.register_16bit.as_ref().unwrap(),
        self.data_8bit.as_ref().unwrap(),
        self.data_16bit.as_ref().unwrap(),
        self.addr_8bit.as_ref().unwrap(),
        self.addr_16bit.as_ref().unwrap(),
        self.pc_relative_8bit.as_ref().unwrap(),
        self.condition_code.as_ref().unwrap(),)
    }
}

impl Default for Opcode {
    fn default() -> Self { Opcode::NOP }
}

impl Instruction{
    pub fn new() -> Self{
        let operand1 = Operand::new();
        let operand2 = Operand::new();
        Instruction{
            binary_value: 0x00,
            opcode: Opcode::LD,
            operand1: Some(operand1),
            operand2: Some(operand2),
        }
    }
}

impl fmt::Display for Instruction{

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Binary value: {:0X?}\n{:?}\n{:?}\n{:?}\n", 
        self.binary_value,
        self.opcode,
        self.operand1.as_ref().unwrap(),
        self.operand2.as_ref().unwrap())
    }
}