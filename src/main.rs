use clap::Parser;
use std::fs::File;
use std::io::Read;

mod disassembler;
mod cpu;
mod mem;
mod instruction;

use instruction::Instruction;
use disassembler::Disassembler;
use instruction::Opcode;

//uncomment below for single instruction debug
// use instruction::Operand;
// use cpu::Register8Bit;

#[derive(Parser, Debug)]
struct Args {
    /// Path to the ROM
    #[clap(long)]
    rom_path: String,
}
fn main() -> Result<(), std::io::Error> {
    let args = Args::parse();
    let mut file = File::open(&args.rom_path)?;
    // println!("file has {} bytes!", file.metadata().unwrap().len());
    let mut contents = Vec::new();
    let mut disassembled = String::new();
    file.read_to_end(&mut contents)?;

    //uncomment below for single instruction debug
    // let op1 = Operand::new();
    // let op2 = Operand::new();
    // let instruction = Instruction::new(0x69, Opcode::LDH)
    // .add_operand(1, op1.add_addr_8bit(0x69))
    // .add_operand(2, op2.add_reg_8bit(Register8Bit::A));
    // println!("{}", instruction);
    
    let disas = Disassembler::new(contents);
    disas.convert_to_asm(&mut disassembled);
    println!("{}", disassembled);
    // for (line_num, byte) in contents.iter().enumerate() {
    //     println!("Line {:#04X?} : {:#04X?}", line_num, byte);
    // }
    Ok(())
}
