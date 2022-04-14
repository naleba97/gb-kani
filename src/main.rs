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
    let disas = Disassembler::new(contents);
    disas.convert_to_asm(&mut disassembled);
    println!("{}", disassembled);
    Ok(())
}
