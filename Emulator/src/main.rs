use std::ops::ControlFlow;
use std::env;
use std::fs::File;
use std::path::Path;
use std::io::{Read, Write, stdin, stdout};
use colored::Colorize;

pub mod instructions;
pub mod registers;
use registers::*;
pub mod cpu_cycle;
use cpu_cycle::{fetch, decode, execute};
pub mod cli;
use cli::CLI;

fn main() {
    let cli = CLI::new(env::args().collect());

    let mut input = String::new();
    let stdin = stdin();
    let mut stdout = stdout();

    // Declare hardware components
    let mut regs = Registers::new();  // Registers
    let mut mem = Vec::new();         // Main memory (16-bit words)

    // Load the bytes of the binary file into a vector (Main memory)
    load_program(&cli.file_path.unwrap(), &mut mem);

    if cli.debug {
        CLI::print_debug_welcome();
    }
 
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

        if cli.debug {
            stdin.read_line(&mut input).unwrap();
            CLI::clear_screen();
            CLI::print_debug_welcome();
            CLI::print_curr_instruction(opcode);
            regs.print();
            stdout.flush().unwrap();
        }
    }

    println!("{}", "\nProgram Halted Normaly\n".green().bold());
    if !cli.debug {regs.print()}
    stdin.read_line(&mut input).unwrap();
}

// *************************** HELPER FUNCTIONS **************************** //

// Loads contents of binary executable file into a vector of
// 16-bit words that will act as main memory
fn load_program(bin_path: &String, mem: &mut Vec<u16>) -> () {
    let path = Path::new(bin_path);
    let mut file = File::open(path).expect("Can not find file");
    
    // Read entire file into a byte array
    let mut bytes: Vec<u8> = Vec::new();
    file.read_to_end(&mut bytes).expect("Could not read file");
    
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
