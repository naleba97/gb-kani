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
    SET,
    HALT
}

#[derive(Debug, Copy, Clone)]
pub struct Operand{
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
            register_8bit: None,
            register_16bit: None,
            data_8bit: None,
            data_16bit: None,
            addr_8bit: None,
            addr_16bit: None,
            pc_relative_8bit: None,
            condition_code: None,
        }
    }
    
    pub fn get_valid_field(self) -> String{
        match (&self.register_8bit, &self.register_16bit, &self.data_8bit, &self.data_16bit, &self.addr_8bit, &self.addr_16bit, &self.pc_relative_8bit, &self.condition_code){
            (Some(Register8Bit), ..) => format!("{:?}", self.register_8bit.unwrap()), 
            (_, Some(Register16Bit), ..) => format!("{:?}", self.register_16bit.unwrap()), 
            (_,_, Some(u8), ..) => format!("{:?}", self.data_8bit.unwrap()),
            (_,_,_, Some(u8), ..) => format!("{:?}", self.data_16bit.unwrap()),
            (_,_,_,_, Some(u16), ..) => format!("{:?}", self.addr_8bit.unwrap()),
            (_,_,_,_,_, Some(u16), ..) => format!("{:?}", self.addr_16bit.unwrap()),
            (_,_,_,_,_,_,Some(i8), _) => format!("{:?}", self.pc_relative_8bit.unwrap()),
            (.., Some(ConditionCode)) => format!("{:?}", self.condition_code.unwrap()),
            _ => format!(""),
        }
    }
    
    pub fn add_reg_8bit(self, reg_8bit: Register8Bit) -> Self {
        Operand { register_8bit: Some(reg_8bit), ..self } 
    }

    pub fn add_reg_16bit(self, reg_16bit: Register16Bit) -> Self {
        Operand { register_16bit: Some(reg_16bit), ..self } 
    }

    pub fn add_data_8bit(self, d8: u8) -> Self {
        Operand { data_8bit: Some(d8), ..self } 
    }

    pub fn add_data_16bit(self, d16: u16) -> Self {
        Operand { data_16bit: Some(d16), ..self } 
    }

    pub fn add_addr_8bit(self, a8: u8) -> Self {
        Operand { addr_8bit: Some(a8), ..self } 
    }

    pub fn add_addr_16bit(self, a16: u16) -> Self {
        Operand { addr_16bit: Some(a16), ..self } 
    }

    pub fn add_pc_rel_8bit(self, r8: i8) -> Self {
        Operand { pc_relative_8bit: Some(r8), ..self } 
    }

    pub fn add_cc(self, cc: ConditionCode) -> Self {
        Operand { condition_code: Some(cc), ..self } 
    }

}

#[derive(Default)]
pub struct Instruction {
    pub binary_value: u8,
    opcode: Opcode,
    operand1: Option<Operand>,
    operand2: Option<Operand>,
}

impl fmt::Display for Operand{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.get_valid_field())
    }
}

impl Default for Opcode {
    fn default() -> Self { Opcode::NOP }
}

impl Instruction{
    pub fn new(byte: u8, opcode_type: Opcode) -> Self {
        let op = Operand::new();
        Instruction{ 
            binary_value: byte, 
            opcode: opcode_type,
            operand1: None,
            operand2: None,
        }
    }
    pub fn add_operand(self, operand_number: u8, operand: Operand) -> Self {
        match operand_number{
            1 => Instruction{operand1: Some(operand), ..self},
            2 => Instruction{operand2: Some(operand), ..self},
            _ => self,
        }
    }

}

impl fmt::Display for Instruction{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut instruction_to_print = String::new();
        instruction_to_print.push_str(&format!("{:#04X?} : {:?}", self.binary_value, self.opcode));
        
        match (&self.operand1, &self.operand2) {
            (None, None) => (),
            (Some(Operand), None) => instruction_to_print.push_str(&format!(" {}", self.operand1.as_ref().unwrap())),
            (Some(Operand), _) => instruction_to_print.push_str(&format!(" {}, {}", self.operand1.as_ref().unwrap(), self.operand2.as_ref().unwrap())),
            _ => ()
        }
        write!(f, "{}", instruction_to_print)
    }
}