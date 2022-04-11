use std::fmt::{Write, Display};

pub struct Disassembler {
    rom: Vec<u8>,
}
//16 bit addr, 8 bit locations

//0x41 == LD B C -> B = C if opcode == 0x41
// 0b01XXXYYY, where XXX is DST and YYY is SRC
// 0b01_000_001 == 0x41, meaning B == 000, C == 001
// AF, BC, DE, HL, SP, PC
//0b00XXX110 -> 0x06 0000_0110 XXX is B, n is the next byte

//0x3e
//0011_1110
//00XXX110 XXX == 111 111 == A

//0b01XX_XXXX -> LD r r'
//0b01XX
//0b00XX_XXXX -> LD r contents_of(r')

impl Disassembler {
    pub fn new(rom: Vec<u8>) -> Self {
        Disassembler { 
            rom: rom
        }
    }

    pub fn convert_to_asm(&self, writer: &mut impl Write) -> std::fmt::Result {  
        for byte in &self.rom {
            let components = OpComponents::from_byte(byte);
            match components.t { 
                0b00 => 
                    match components.x {
                        0b000..=0b011 =>   
                            match components.y{
                                
                                _ => (),
                            }
                        _ => (),
                    }
                0b01 => 
                    if components.x == 6 && components.y == 6 { //HALT
                        write!(writer, "{:#04X?} : HALT\n", byte)? 
                    }
                    else { 
                        write!(writer, "{:#04X?} : LD {} {} \n", byte, 
                        OpComponents::convert_byte_to_reg_name(&components.x), 
                        OpComponents::convert_byte_to_reg_name(&components.y))? 
                    }
                _ => (),//write!(writer, "{:#04X?}, ", byte)?,
            }
        }
        Ok(())
    }
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

struct OpComponents {
    t: u8,
    x: u8,
    y: u8,
//     p: u8,
//     q: bool,
}

impl OpComponents {
    fn from_byte(byte: &u8) -> OpComponents {
        OpComponents {
            t: (byte & 0b1100_0000) >> 6,
            x: (byte & 0b0011_1000) >> 3,
            y: (byte & 0b0000_0111),
            // p: (byte & 0b0011_0000) >> 4,
            // q: (byte & 0b0000_1000) != 0,
        }
    }

    fn convert_byte_to_reg_name(byte: &u8) -> String{
        let mut register_name = String::new();
        match byte {
            0b000 => register_name = String::from("B"),
            0b001 => register_name = String::from("C"),
            0b010 => register_name = String::from("D"),
            0b011 => register_name = String::from("E"),
            0b100 => register_name = String::from("H"),
            0b101 => register_name = String::from("L"),
            0b110 => register_name = String::from("(HL)"),
            0b111 => register_name = String::from("A"),
            _ => register_name = String::from("NOT VALID REGISTER"),
        }
        return register_name;
    }

    // fn is_load(&self, byte: &u8) -> Operation {
    //     match self.t {
    //         0 => Operation::LOAD,
    //     }
    // }
}