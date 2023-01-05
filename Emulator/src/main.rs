use std::fs::File;
use std::path::Path;
use std::io::Read;
use num_traits::FromPrimitive;

pub mod instructions;
use instructions::{Opcode, execute};
pub mod registers;
use registers::Registers;

fn main() {
    // Declare hardware components
    let mut pc_reg: u16 = 0;             // Program Counter 
    let mut ir_reg: u16;                 // Instruction Register
    let mut gp_regs = Registers::new();  // General Purpose Registers
    let mut ram = Vec::new();            // Main memory (16-bit words)

    // Load the bytes of the binary file into a vector to work on
    load_program("file.bin", &mut ram);

    loop {
        // Fetch Stage
        ir_reg = fetch(pc_reg, &ram);

        // Decode Stage
        let opcode = decode(ir_reg);

        // Execute Stage 
        match FromPrimitive::from_u8(opcode) {
            Some(Opcode::MovIm) => execute::mov_im(ir_reg, &mut gp_regs),
            Some(Opcode::MovRg) => (),
            _ => break,
        }
    }

    pc_reg += 1;
    println!("{}", pc_reg);
}

// Fetch stage: returns the instruction in RAM specified by pc
fn fetch (pc: u16, ram: &Vec<u16>) -> u16 {
    ram[pc as usize]
}

// Decode stage: Obtains a 5-bit value from 16-bit register to
//               use as opcode
fn decode (ir_reg: u16) -> u8 {
    const OPCODE_SHIFT_OFFSET: u16 = 11;
    // 16-bit register offseted by 11 bits to retrieve
    // only the value of the 5 bits of the opcode
    (ir_reg << OPCODE_SHIFT_OFFSET) as u8
}

// Loads contents of binary executable file into a vector of
// 16-bit words that will act as main memory
fn load_program(bin_path: &str, buf: &mut Vec<u16>) -> () {
    let path = Path::new(bin_path);
    let mut file = File::open(path).expect("Can not find file");
    
    // Read entire file into a byte array
    let mut bytes: Vec<u8> = Vec::new();
    file.read_to_end(&mut bytes).expect("Could not read file");

    let mut i = 0; // Index to each memory word (2 bytes)
    while i < bytes.len() {
        // Convert each two-byte group into a 16-bit word
        buf.push(u16::from_be_bytes([bytes[i], bytes[i+1]]));
        i += 2;
    }
}
