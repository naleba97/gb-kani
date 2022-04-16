use std::fmt::{Display, Write};
use std::cmp::Ordering;

use crate::instruction::Instruction;
use crate::instruction::Opcode;
use crate::instruction::Operand;

use crate::cpu::Register8Bit;
use crate::cpu::Register16Bit;
use crate::cpu::ConditionCode;

macro_rules! set_nop {
  ($instruction: expr) => {$instruction = Instruction::new(0x00, Opcode::NOP); }
}

macro_rules! fetch_next_byte{
    ($bytes: expr, $byte1: expr) => {
        $byte1 = *$bytes.next().unwrap()
    };

    ($bytes: expr, $byte1: expr, $byte2: expr) => {
        $byte1 = *$bytes.next().unwrap();
        $byte2 = *$bytes.next().unwrap();
    }
}

macro_rules! increment_pc {
    ($num_bytes: expr, $pc: expr) => {
        $pc += $num_bytes
    }
}

fn create_16bit_from_2_bytes(byte1: u8, byte2: u8){
    
}

pub struct Disassembler {
    rom: Vec<u8>,
}

impl Disassembler {
    pub fn new(rom: Vec<u8>) -> Self {
        Disassembler { rom: rom }
    }

    pub fn convert_to_asm(&self, writer: &mut impl Write) -> std::fmt::Result {
        let mut num_bytes = self.rom.len();
        let mut instruction = Instruction::new(0x00, Opcode::NOP);
        let (mut byte1, mut byte2): (u8, u8);
        let mut operand2: u8;
        let op1 = Operand::new();
        let op2 = Operand::new();
        let mut prev_pc = 0 as u16; //pc is increment 1 after the instruction pc + r8 instructions are 2 bytes so we good??
        let mut next_pc = 0 as u16;
        // for (bytes, byte) in contents.iter().enumerate {

        // bytes.nth(0); //nth 0 for consuming next, nth 1 for consuming next 2
        // println!("NICK: {}", bytes.size_hint().0);
        let mut bytes = self.rom.iter();
    
        // for (byte_num, byte) in self.rom.iter().enumerate() {
        //TODO: to merge Same<identifier>
        while let Some(byte) = bytes.next(){
            // write!(writer,"{:#04X?}\n", byte);
            prev_pc = next_pc;
            let mut components = OpComponents::from_byte(byte);
            let mut byte = *byte;
            match components.t {
                0b00 => {
                    match components.x {
                        0b000|0b010|0b100|0b110 => {
                            match components.y{
                                0b000 =>{
                                    match components.x {
                                        0b100|0b110 => {
                                            fetch_next_byte!(bytes, byte1);
                                            increment_pc!(2, next_pc);
                                            instruction = Instruction::new(byte, Opcode::JR)
                                            .add_operand(1, op1.add_cc(get_cc_operand(&components.x)))
                                            .add_operand(2, op2.add_pc_rel_8bit(byte1 as i8));
                                        },
                                        _ => set_nop!(instruction),
                                    }
                                }
                                0b001 =>{
                                    fetch_next_byte!(bytes, byte1, byte2);
                                    increment_pc!(3,next_pc);
                                    instruction = Instruction::new(byte, Opcode::LD)
                                    .add_operand(1, op1.add_reg_16bit(get_16bit_reg_name(&components.z, RegType::SP)))
                                    .add_operand(2, op2.add_data_16bit_from_bytes(byte1, byte2));
                                },
                                0b010 =>{ 
                                    increment_pc!(1,next_pc);
                                    instruction = Instruction::new(byte, Opcode::LD)
                                    .add_operand(1, op1.add_reg_16bit(get_16bit_reg_name(&components.z, RegType::HL)).add_addr_trait())
                                    .add_operand(2, op2.add_reg_8bit(Register8Bit::A))
                                } 
                                0b011 =>{ 
                                    increment_pc!(1,next_pc);
                                    instruction = Instruction::new(byte, Opcode::INC)
                                    .add_operand(1, op1.add_reg_16bit(get_16bit_reg_name(&components.z, RegType::SP)))
                                } 
                                0b100 =>{ //TODO: SameB
                                    increment_pc!(1,next_pc);
                                    instruction = Instruction::new(byte, Opcode::INC)
                                    .add_operand(1, op1.add_reg_8bit(get_8bit_reg_name(&components.x)).add_addr_trait())
                                }
                                0b101 =>{ //TODO: SameD
                                    increment_pc!(1,next_pc);
                                    instruction = Instruction::new(byte, Opcode::DEC)
                                    .add_operand(1, op1.add_reg_8bit(get_8bit_reg_name(&components.x)).add_addr_trait())
                                }
                                0b110 =>{ //TODO: SameA
                                    fetch_next_byte!(bytes, byte1);
                                    increment_pc!(2,next_pc);
                                    instruction = Instruction::new(byte, Opcode::LD)
                                    .add_operand(1, op1.add_reg_8bit(get_8bit_reg_name(&components.x)).add_addr_trait())
                                    .add_operand(2, op2.add_data_8bit(byte1));
                                },
                                _ => set_nop!(instruction),
                            }
                        }
                        0b001|0b011|0b101|0b111 => {
                            match components.y{
                                0b010 =>{
                                    increment_pc!(1,next_pc);
                                    instruction = Instruction::new(byte, Opcode::LD)
                                    .add_operand(1, op1.add_reg_8bit(Register8Bit::A))
                                    .add_operand(2, op2.add_reg_16bit(get_16bit_reg_name(&components.z, RegType::HL)).add_addr_trait());
                                },
                                0b100 =>{ //TODO: SameB
                                    increment_pc!(1,next_pc);
                                    instruction = Instruction::new(byte, Opcode::INC)
                                    .add_operand(1, op1.add_reg_8bit(get_8bit_reg_name(&components.x)).add_addr_trait())
                                },
                                0b101 =>{ //TODO: SameD
                                    increment_pc!(1,next_pc);
                                    instruction = Instruction::new(byte, Opcode::DEC)
                                    .add_operand(1, op1.add_reg_8bit(get_8bit_reg_name(&components.x)).add_addr_trait())
                                },
                                0b110 =>{ //TODO: SameA
                                    fetch_next_byte!(bytes, byte1);
                                    increment_pc!(2,next_pc);
                                    instruction = Instruction::new(byte, Opcode::LD)
                                    .add_operand(1, op1.add_reg_8bit(get_8bit_reg_name(&components.x)).add_addr_trait())
                                    .add_operand(2, op2.add_data_8bit(byte1));
                                }
                                _ => set_nop!(instruction),
                            }
                        }
                        _ => set_nop!(instruction),
                    }
                },
                0b01 => {
                    if components.x == 6 && components.y == 6 { 
                        increment_pc!(1,next_pc);
                        instruction = Instruction::new(byte, Opcode::HALT);
                    }
                    else { 
                        increment_pc!(1,next_pc);
                        instruction = Instruction::new(byte, Opcode::LD)
                        .add_operand(1, op1.add_reg_8bit(get_8bit_reg_name(&components.x)).add_addr_trait())
                        .add_operand(2, op2.add_reg_8bit(get_8bit_reg_name(&components.y)).add_addr_trait());
                    }
                },
                0b10 => {  //Arithmetic
                    increment_pc!(1,next_pc);
                    match components.x{
                        0b000 =>{
                            instruction = Instruction::new(byte, Opcode::ADD)
                            .add_operand(1, op1.add_reg_8bit(Register8Bit::A))
                            .add_operand(2, op2.add_reg_8bit(get_8bit_reg_name(&components.y)).add_addr_trait());
                        }
                        0b001 =>{
                            instruction = Instruction::new(byte, Opcode::ADC)
                            .add_operand(1, op1.add_reg_8bit(Register8Bit::A))
                            .add_operand(2, op2.add_reg_8bit(get_8bit_reg_name(&components.y)).add_addr_trait());
                        }
                        0b010 =>{
                            instruction = Instruction::new(byte, Opcode::SUB)
                            .add_operand(1, op1.add_reg_8bit(get_8bit_reg_name(&components.y)).add_addr_trait());
                        }
                        0b011 =>{
                            instruction = Instruction::new(byte, Opcode::SBC)
                            .add_operand(1, op1.add_reg_8bit(get_8bit_reg_name(&components.y)).add_addr_trait());
                        }
                        0b100 =>{
                            instruction = Instruction::new(byte, Opcode::AND)
                            .add_operand(1, op1.add_reg_8bit(get_8bit_reg_name(&components.y)).add_addr_trait());
                        }
                        0b101 =>{
                            instruction = Instruction::new(byte, Opcode::XOR)
                            .add_operand(1, op1.add_reg_8bit(get_8bit_reg_name(&components.y)).add_addr_trait());
                        }
                        0b110 =>{
                            instruction = Instruction::new(byte, Opcode::OR)
                            .add_operand(1, op1.add_reg_8bit(get_8bit_reg_name(&components.y)).add_addr_trait());
                        }
                        0b111 =>{
                            instruction = Instruction::new(byte, Opcode::CP)
                            .add_operand(1, op1.add_reg_8bit(get_8bit_reg_name(&components.y)).add_addr_trait());
                        }
                        _ => set_nop!(instruction),
                    }
                },
                0b11 => {
                    match components.x {
                        0b001|0b011|0b101|0b111 => match components.y {
                            0b011 => { //PREFIX
                                fetch_next_byte!(bytes, byte1);
                                increment_pc!(2,next_pc);
                                let prefix_opcode = get_prefix_opcode(&byte1);
                                components = OpComponents::from_byte(&byte1);
                                match prefix_opcode {
                                    Opcode::BIT|Opcode::RES|Opcode::SET => {
                                        instruction = Instruction::new(byte1, prefix_opcode)
                                        .add_operand(1, op1.add_prefix_num(components.x))
                                        .add_operand(2, op2.add_reg_8bit(get_8bit_reg_name(&components.y)).add_addr_trait());
                                    }
                                    _ => {
                                        instruction = Instruction::new(byte, prefix_opcode)
                                        .add_operand(1, op1.add_reg_8bit(get_8bit_reg_name(&components.y)).add_addr_trait())
                                    }
                                }
                            },
                            0b110 =>{
                                fetch_next_byte!(bytes, byte1);
                                increment_pc!(2,next_pc);
                                match components.z{
                                    0b00 => {
                                        instruction = Instruction::new(byte, Opcode::ADC)
                                        .add_operand(1, op1.add_reg_8bit(Register8Bit::A))
                                        .add_operand(2, op2.add_data_8bit(byte1))
                                    }
                                    0b01 => {
                                        instruction = Instruction::new(byte, Opcode::SBC)
                                        .add_operand(1, op1.add_reg_8bit(Register8Bit::A))
                                        .add_operand(2, op2.add_data_8bit(byte1))
                                    }
                                    0b10 => {
                                        instruction = Instruction::new(byte, Opcode::XOR)
                                        .add_operand(1, op1.add_data_8bit(byte1))
                                    }
                                    0b11 => {
                                        instruction = Instruction::new(byte, Opcode::CP)
                                        .add_operand(1, op1.add_data_8bit(byte1))
                                    }
                                    _ => set_nop!(instruction),
                                }
                            }
                            0b100 => { //TODO: SameC
                                fetch_next_byte!(bytes, byte1, byte2);
                                increment_pc!(3,next_pc);
                                instruction = Instruction::new(byte, Opcode::CALL)
                                .add_operand(1, op1.add_cc(get_cc_operand(&components.x)))
                                .add_operand(2, op2.add_addr_16bit(byte1, byte2).remove_addr_trait_from_addr());
                            }
                            0b101 => { 
                                fetch_next_byte!(bytes, byte1, byte2);
                                increment_pc!(3,next_pc);
                                instruction = Instruction::new(byte, Opcode::CALL)
                                .add_operand(1, op1.add_addr_16bit(byte1, byte2).remove_addr_trait_from_addr())
                            }
                            _ => set_nop!(instruction),
                        }
                        0b100|0b110 => {
                            match components.y{
                                0b000 => {
                                    fetch_next_byte!(bytes, byte1);
                                    increment_pc!(2,next_pc);
                                    if(components.x == u8::from(0b100)) {
                                        instruction = Instruction::new(byte, Opcode::LDH)
                                        .add_operand(1, op1.add_addr_8bit(byte1))
                                        .add_operand(2, op2.add_reg_8bit(Register8Bit::A));
                                    }
                                    else{
                                        instruction = Instruction::new(byte, Opcode::LDH)
                                        .add_operand(1, op1.add_reg_8bit(Register8Bit::A))
                                        .add_operand(2, op2.add_addr_8bit(byte1));
                                    }
                                }
                                0b010 => {
                                    increment_pc!(1,next_pc);
                                    let (mut op1_reg, mut op2_reg): (Register8Bit, Register8Bit);
                                    if(components.x == u8::from(0b100)) {
                                        op1_reg = Register8Bit::C;
                                        op2_reg = Register8Bit::A;
                                    }
                                    else{
                                        op1_reg = Register8Bit::A;
                                        op2_reg = Register8Bit::C;
                                    }
                                    instruction = Instruction::new(byte, Opcode::LD)
                                    .add_operand(1, op1.add_reg_8bit_addr(op1_reg))
                                    .add_operand(2, op2.add_reg_8bit(op2_reg));
                                }
                                _ => set_nop!(instruction),
                            }
                                
                        }
                        0b000|0b010 => {
                            match components.y{
                                0b010 => {
                                    fetch_next_byte!(bytes, byte1, byte2);
                                    increment_pc!(3,next_pc);
                                    instruction = Instruction::new(byte, Opcode::JP)
                                    .add_operand(1, op1.add_cc(get_cc_operand(&components.x)))
                                    .add_operand(2, op2.add_addr_16bit(byte1, byte2));
                                }
                                0b100 => { //TODO: SameC
                                    fetch_next_byte!(bytes, byte1, byte2);
                                    increment_pc!(3,next_pc);
                                    instruction = Instruction::new(byte, Opcode::CALL)
                                    .add_operand(1, op1.add_cc(get_cc_operand(&components.x)))
                                    .add_operand(2, op2.add_addr_16bit(byte1, byte2).remove_addr_trait_from_addr());
                                }
                                _ => set_nop!(instruction),
                            }
                        }
                        _ => set_nop!(instruction),
                    }
                },
                _ => set_nop!(instruction),
            }
            if(instruction.binary_value != 0x00){
                write!(writer, "Prev_PC: {:#06X?} | Next_PC {:#06X?} | Instruction: {}\n", prev_pc, next_pc, instruction);
                set_nop!(instruction);
            }
        }
        Ok(())
    }
}

enum RegType {
    SP,
    HL
}

struct OpComponents {
    t: u8,
    x: u8,
    y: u8,
    z: u8,
}

pub fn get_prefix_opcode(byte: &u8) -> Opcode{
    match byte{
        0b0000_0000..=0b0000_0111 => Opcode::RLC, 
        0b0000_1000..=0b0000_1111 => Opcode::RRC,
        0b0001_0000..=0b0001_0111 => Opcode::RL,
        0b0001_1000..=0b0001_1111 => Opcode::RR, 
        0b0010_0000..=0b0010_0111 => Opcode::SLA, 
        0b0010_1000..=0b0010_1111 => Opcode::SRA, 
        0b0011_0000..=0b0011_0111 => Opcode::SWAP,
        0b0011_1000..=0b0011_1111 => Opcode::SRL,
        0b0100_0000..=0b0111_1111 => Opcode::BIT,
        0b1000_0000..=0b1011_1111 => Opcode::RES,
        0b1100_0000..=0b1111_1111 => Opcode::SET,
        _ => panic!(),
    }
}

pub fn get_8bit_reg_name(byte: &u8) -> Register8Bit {
    match byte {
        0b000 => Register8Bit::B,
        0b001 => Register8Bit::C,
        0b010 => Register8Bit::D,
        0b011 => Register8Bit::E,
        0b100 => Register8Bit::H,
        0b101 => Register8Bit::L,
        0b110 => Register8Bit::HL,
        0b111 => Register8Bit::A,
        _ => panic!(),
    }
}

fn get_16bit_reg_name(byte: &u8, reg_type: RegType) -> Register16Bit {
    match (byte, reg_type) {
        (0b00, _) => Register16Bit::BC,
        (0b01, _) => Register16Bit::DE,
        (0b10, RegType::HL) => Register16Bit::HLp,
        (0b11, RegType::HL) => Register16Bit::HLm,
        (0b10, RegType::SP) => Register16Bit::HL,
        (0b11, RegType::SP) => Register16Bit::SP,
        _ => panic!(),
    }
}

fn get_cc_operand(byte: &u8) -> ConditionCode {
    match byte {
        0b000|0b100 => ConditionCode::NZ,
        0b001|0b101 => ConditionCode::Z,
        0b010|0b110 => ConditionCode::NC,
        0b011|0b111 => ConditionCode::C,
        _ => panic!(),
    }
}

impl OpComponents {
    fn from_byte(byte: &u8) -> OpComponents {
        OpComponents {
            t: (byte & 0b1100_0000) >> 6,
            x: (byte & 0b0011_1000) >> 3,
            y: (byte & 0b0000_0111),
            z: (byte & 0b0011_0000) >> 4,
            // q: (byte & 0b0000_1000) != 0,
        }
    }
    
  

    fn process_prefix(byte: &u8) -> &str {
        return "PREFIX" 
    }

    // fn is_load(&self, byte: &u8) -> Operation {
    //     match self.t {
    //         0 => Operation::LOAD,
    //     }
    // }
}

// enum Operation {
//     LOAD,
//     STORE,
//     ADD,
//     SUB,
// }


// impl Display for Operation {
//     fn fmt(&self, f: &mut Formatter<'_>) -> Result {
//         match {
//             LOAD => f.write("LD {} {}", )
//         }
//     }
// }