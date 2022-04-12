use std::fmt::{Display, Write};

pub struct Disassembler {
    rom: Vec<u8>,
}

impl Disassembler {
    pub fn new(rom: Vec<u8>) -> Self {
        Disassembler { rom: rom }
    }

    pub fn convert_to_asm(&self, writer: &mut impl Write) -> std::fmt::Result {
        let mut single_operand_to_process = false;
        let mut multiple_operands_to_process = 0;
        let mut prefix_operand_to_process = false;
        let mut asm_to_print = String::new();
        let mut immediate_value: u16 = 0;
        for byte in &self.rom {
            let components = OpComponents::from_byte(byte);
            // write!(writer, "{:#04X?}\n", byte)?
            if multiple_operands_to_process > 0 {
                if multiple_operands_to_process == 2 {
                    immediate_value += u16::from(*byte);
                } else if multiple_operands_to_process == 1 {
                    immediate_value += u16::from(*byte) << 8;
                    asm_to_print.push_str(&format!(" ${:#06X?}", immediate_value));
                    immediate_value = 0;
                    write!(writer, "{}\n", asm_to_print)?;
                    asm_to_print = String::from("");
                }
                multiple_operands_to_process -= 1;
            } else if single_operand_to_process {
                immediate_value += u16::from(*byte);
                asm_to_print.push_str(&format!(" ${:#04X?}", immediate_value));
                immediate_value = 0;
                write!(writer, "{}\n", asm_to_print)?;
                asm_to_print = String::from("");
                single_operand_to_process = false;
            } else if prefix_operand_to_process {
                write!(writer, "{:#04X?} {} \n", byte, OpComponents::process_prefix(&byte))?;
                prefix_operand_to_process = false;
            } else {
                match components.t {
                    0b00 => match components.x {
                        //MISC
                        0b000 | 0b010 | 0b100 | 0b110 => match components.y {
                            0b001 => {
                                asm_to_print = format!(
                                    "{:#04X?} : LD {},",
                                    byte,
                                    OpComponents::convert_byte_to_16bit_reg_name(
                                        &components.z,
                                        RegType::SP
                                    )
                                );
                                multiple_operands_to_process = 2;
                            }
                            0b010 => {
                                //special loads HL-/HL+
                                write!(
                                    writer,
                                    "{:#04X?} : LD ({}), A\n",
                                    byte,
                                    OpComponents::convert_byte_to_16bit_reg_name(
                                        &components.z,
                                        RegType::HL)
                                    )?
                                
                            }
                            0b110 => {
                                //merge?
                                asm_to_print = format!(
                                    "{:#04X?} : LD {},",
                                    byte,
                                    OpComponents::convert_byte_to_8bit_reg_name(&components.x)
                                );
                                single_operand_to_process = true;
                            }
                            _ => (),
                        },
                        0b001 | 0b011 | 0b101 | 0b111 =>
                        //same as above, how merge?
                        {
                            match components.y {
                                0b110 => {
                                    asm_to_print = format!(
                                        "{:#04X?} : LD {},",
                                        byte,
                                        OpComponents::convert_byte_to_8bit_reg_name(&components.x)
                                    );
                                    single_operand_to_process = true;
                                }
                                _ => (),
                            }
                        }
                        _ => (),
                    },
                    0b01 => {
                        //LDs and HALT
                        if components.x == 6 && components.y == 6 {
                            //HALT
                            write!(writer, "{:#04X?} : HALT\n", byte)?
                        } else {
                            write!(
                                writer,
                                "{:#04X?} : LD {}, {} \n",
                                byte,
                                OpComponents::convert_byte_to_8bit_reg_name(&components.x),
                                OpComponents::convert_byte_to_8bit_reg_name(&components.y)
                            )?
                        }
                    }
                    0b10 => {
                        //Arithmetic, merge into more variable lol
                        match components.x {
                            0b000 => {
                                match components.y {
                                    0b0000..=0b0111 => {
                                        //Adds
                                        write!(
                                            writer,
                                            "{:#04X?} : ADD A, {} \n",
                                            byte,
                                            OpComponents::convert_byte_to_8bit_reg_name(
                                                &components.y
                                            )
                                        )?
                                    }
                                    _ => (),
                                }
                            }
                            0b001 => {
                                //TODO same as above minus ADC
                                match components.y {
                                    0b0000..=0b0111 => {
                                        //Adds
                                        write!(
                                            writer,
                                            "{:#04X?} : ADC A, {} \n",
                                            byte,
                                            OpComponents::convert_byte_to_8bit_reg_name(
                                                &components.y
                                            )
                                        )?
                                    }
                                    _ => (),
                                }
                            }
                            0b010 => match components.y {
                                0b0000..=0b0111 => write!(
                                    writer,
                                    "{:#04X?} : SUB {} \n",
                                    byte,
                                    OpComponents::convert_byte_to_8bit_reg_name(&components.y)
                                )?,
                                _ => (),
                            },
                            0b011 => match components.y {
                                0b0000..=0b0111 => write!(
                                    writer,
                                    "{:#04X?} : SUB A, {} \n",
                                    byte,
                                    OpComponents::convert_byte_to_8bit_reg_name(&components.y)
                                )?,
                                _ => (),
                            },
                            0b100 => match components.y {
                                0b0000..=0b0111 => write!(
                                    writer,
                                    "{:#04X?} : AND {} \n",
                                    byte,
                                    OpComponents::convert_byte_to_8bit_reg_name(&components.y)
                                )?,
                                _ => (),
                            },
                            0b101 => match components.y {
                                0b0000..=0b0111 => write!(
                                    writer,
                                    "{:#04X?} : XOR {} \n",
                                    byte,
                                    OpComponents::convert_byte_to_8bit_reg_name(&components.y)
                                )?,
                                _ => (),
                            },
                            0b110 => match components.y {
                                0b0000..=0b0111 => write!(
                                    writer,
                                    "{:#04X?} : OR {} \n",
                                    byte,
                                    OpComponents::convert_byte_to_8bit_reg_name(&components.y)
                                )?,
                                _ => (),
                            },
                            0b111 => match components.y {
                                0b0000..=0b0111 => write!(
                                    writer,
                                    "{:#04X?} : CP {} \n",
                                    byte,
                                    OpComponents::convert_byte_to_8bit_reg_name(&components.y)
                                )?,
                                _ => (),
                            },
                            _ => (),
                        }
                    }
                    0b11 => match components.x {
                        0b001 => match components.y {
                            0b011 => { //PREFIX
                                prefix_operand_to_process = true;
                            }
                            _ => (),
                        }
                        _ => (),
                    }
                    _ => (), //write!(writer, "{:#04X?}, ", byte)?,
                }
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
    
    fn convert_byte_to_16bit_reg_name(byte: &u8, reg_type: RegType) -> &str {
        match (byte, reg_type) {
            (0b00, _) => "BC",
            (0b01, _) => "DE",
            (0b10, RegType::HL) => "HL-",
            (0b11, RegType::HL) => "HL+",
            (0b10, RegType::SP) => "HL",
            (0b10, RegType::SP) => "SP",
            _ => "NOT VALID REGISTER",
        }
    }

    fn convert_byte_to_8bit_reg_name(byte: &u8) -> &str {
        match byte {
            0b000 => "B",
            0b001 => "C",
            0b010 => "D",
            0b011 => "E",
            0b100 => "H",
            0b101 => "L",
            0b110 => "(HL)",
            0b111 => "A",
            _ => "NOT VALID REGISTER",
        }
    }

    fn process_prefix(byte: &u8) -> &str {
        "PREFIX" 
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