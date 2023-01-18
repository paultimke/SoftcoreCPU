use std::{ops::ControlFlow};
use std::env;
use std::fs::File;
use std::path::Path;
use std::io::Read;
use colored::Colorize;

pub mod instructions;
pub mod registers;
use registers::*;
pub mod cpu_cycle;
use cpu_cycle::{fetch, decode, execute};

fn main() {
    let args: Vec<String> = env::args().collect();
    if !(args.len() == 2 || args.len() == 3) {
        println!("{}: Please pass in 1 binary file as argument", "Error".red());
        return;
    }
    let file_path = &args[1];

    // Declare hardware components
    let mut regs = Registers::new();  // Registers
    let mut mem = Vec::new();         // Main memory (16-bit words)

    // Load the bytes of the binary file into a vector (Main memory)
    load_program(file_path, &mut mem);

    regs.print();

    // TODO: 
    //       - Add feature DEBUG to step through code on each iteration of the loop
    //         by pressing some key (maybe space to continue)
    //       - When on DEBUG, add feature to see register values when pressing
    //         some key (maybe r)
 
    // Fetch-Decode-Execute Cycle
    loop {
        regs.ir = fetch(regs.pc, &mem);
        let opcode = decode(regs.ir);

        match execute(opcode, &mut regs, &mut mem) {
            Some(ControlFlow::Continue(_)) => continue,
            Some(ControlFlow::Break(_)) => break,
            None => ()
        }
        regs.pc += 1;
    }

    println!("{}", "\nProgram Halted Normaly\n".green().bold());
    regs.print();

}

// Loads contents of binary executable file into a vector of
// 16-bit words that will act as main memory
fn load_program(bin_path: &str, mem: &mut Vec<u16>) -> () {
    let path = Path::new(bin_path);
    let mut file = File::open(path).expect("Can not find file");
    
    // Read entire file into a byte array
    let mut bytes: Vec<u8> = Vec::new();
    file.read_to_end(&mut bytes).expect("Could not read file");

    // TODO: Read linker script Config.map to get actual stack size,
    // For now, it will be hardcoded. This process and the linker script
    // will actually need to live in the Assembler module (not the Emulator)
    // but since we don't allocate the full size of RAM here (only what is needed)
    // we do need to know what size to allocate.
    // It will be the programmer's job to set the Stack Pointer
    // to address CODE_SECTION_LENGTH + STACK_LENGTH. In this case 0x15E
    // Sizes specified in words
    const CODE_SECTION_LENGTH: usize = 300;
    const STACK_LENGTH: usize = 50;
    mem.resize(CODE_SECTION_LENGTH + STACK_LENGTH, 0);

    let mut w = 0; // Index to each memory word (2 bytes)
    for idx in 0..(bytes.len()/2) {
        // Convert each two-byte group into a 16-bit word
        mem[idx] = u16::from_be_bytes([bytes[w], bytes[w+1]]);
        w += 2;
    }
}
