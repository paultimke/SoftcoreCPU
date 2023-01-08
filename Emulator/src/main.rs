use std::fs::File;
use std::path::Path;
use std::io::Read;
use num_traits::FromPrimitive;

pub mod instructions;
use instructions::{Opcode, execute};
pub mod registers;
use registers::*;

fn main() {
    // Declare hardware components
    let mut regs = Registers::new();  // Registers
    let mut mem = Vec::new();         // Main memory (16-bit words)

    // Load the bytes of the binary file into a vector (Main memory)
    load_program("file.bin", &mut mem);
 
    // Fetch-Decode-Execute Cycle
    loop {
        // Fetch Stage
        regs.ir = fetch(regs.pc, &mem);

        // Decode Stage
        let opcode = decode(regs.ir);

        // Execute Stage 
        match FromPrimitive::from_u8(opcode) {
            Some(Opcode::MovIm)  => execute::mov_im(&mut regs),
            Some(Opcode::MovRg)  => execute::mov_rg(&mut regs),
            Some(Opcode::Load)   => execute::load(&mut regs, &mem),
            Some(Opcode::LoadRg) => execute::load_rg(&mut regs, &mem),
            Some(Opcode::Store)  => execute::store(&regs, &mut mem),
            Some(Opcode::StrRg)  => execute::store_rg(&regs, &mut mem),
            Some(Opcode::Push)   => execute::push(&mut regs, &mut mem),
            Some(Opcode::Pop)    => execute::pop(&mut regs, &mem),
            Some(Opcode::AddIm)  => execute::add_im(&mut regs),
            Some(Opcode::AddRg)  => execute::add_rg(&mut regs),
            Some(Opcode::SubIm)  => execute::sub_im(&mut regs),
            Some(Opcode::SubRg)  => execute::sub_rg(&mut regs),
            Some(Opcode::ShftL)  => execute::shift_l(&mut regs),
            Some(Opcode::ShftR)  => execute::shift_r(&mut regs),
            Some(Opcode::And)    => execute::and(&mut regs),
            Some(Opcode::Or)     => execute::or(&mut regs),
            Some(Opcode::Not)    => execute::not(&mut regs),
            Some(Opcode::Bal)    => {execute::bal(&mut regs); continue},
            Some(Opcode::Bzr)    => {execute::bzr(&mut regs); continue},
            Some(Opcode::Bng)    => {execute::bng(&mut regs); continue},
            Some(Opcode::Bln)    => {execute::bln(&mut regs); continue},
            Some(Opcode::Ret)    => {execute::ret(&mut regs); continue},
            Some(Opcode::Halt)   => {println!("Program Halted Normally");
                                     break },
            _ => panic!("Unrecognized Opcode"),
        }
        regs.pc += 1;
    }
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
    (ir_reg >> OPCODE_SHIFT_OFFSET) as u8
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
