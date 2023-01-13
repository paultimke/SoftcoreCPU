use colored::Colorize;
use std::collections::HashMap;
use once_cell::sync::Lazy;
use crate::parser::{error_handler, LineError};

// ********************* VARIABLES AND TYPE DEFINITIONS ******************** //

// Instruction Type: Categorizes the types of instructions
// by the number and bit sizes of the fields of the instruction.
// This Enum must be passed as parameter to the encode function
// to determine how the instruction will be encoded.
// Please refer to the CPU or Assembler Reference Manuals
enum InstructionType {
    T1(u8, u8, u8), 
    T2(u8, u8, u8, u8), 
    T3(u8, u8), 
    T4(u8, u8, u8, u8), // Fifth field unused
    T5(u8, u8, u8, u8)  // Fifth field unused
}
const UNUSED: u8 = 0x00; // Default for the Unused field in InstructionType

// Callback function returned to the parser on mnemonic matches to encode
// complete instruction as a byte pair
type EncodeCallback = fn(Vec<String>, usize) -> [u8; 2];

// Look-up table for Mnemonics and corresponding encoder function
pub static MNEMONICS: Lazy<HashMap<&str, EncodeCallback>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("mov", mov as EncodeCallback);
    m.insert("load", mov as EncodeCallback);
    m
});

// Look-up table for existing Registers and their address
pub static REGISTERS: Lazy<HashMap<&str, u8>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("r0", 0);
    m.insert("r1", 1);
    m.insert("r2", 2);
    m.insert("r3", 3);
    m.insert("r4", 4);
    m.insert("r5", 5);
    m.insert("r6", 6);
    m.insert("r7", 7);
    m.insert("fp", 4);
    m.insert("sp", 5);
    m.insert("lr", 6);
    m.insert("mbr", 7);
    m
});

// ************************ PRIVATE HELPER FUNCTIONS *********************** //

// Encodes the passed in values by their InstructionType as specified
// by the reference manual.
fn encode(instr: InstructionType) -> [u8; 2] {
    match instr {
        InstructionType::T1(op,r,c) => {
            let opcode_shift = 11;
            let reg_pos0_shift = 8;
            let imm_shift = 0;
            // Do the sift operations
        }
        InstructionType::T2(op,rp0,rp1,f1) => {

        }
        InstructionType::T3(op,f2) => {

        }
        InstructionType::T4(op,rp0,rp1,rp2) => {

        }
        InstructionType::T5(op,rp0,rp1,c) => {
            
        }
    }
    [1u8, 2u8]
}

fn get_valid_reg(r: &str, curr_line_num: usize) -> u8 {
    match REGISTERS.get(r) {
        Some(val) => *val,
        None => {error_handler(LineError::Unrecognized(String::from(r)), curr_line_num); 0}
    }
}

fn get_valid_imm(c: &str, curr_line_num: usize) -> u8 {
    let imm = match c[1..].parse::<i8>() {
        Ok(val) => val,
        Err(_) => {error_handler(LineError::Unrecognized(String::from(c)), curr_line_num); 0}
    } as u8;
    imm
}

// ********************* MNEMONIC ASSEMBLING FUNCTIONS ********************* //

// MOV Opcode can be one of two variants: Immediate or with Registers
// The kind of variant is determined here by the type of the second argument
pub fn mov(args: Vec<String>, line_number: usize) -> [u8; 2] {
    if args.len() != 2 {
        let msg = format!("{} operation can only have 2 parameters", "mov".bold());
        error_handler(LineError::WrongArgs(msg), line_number)
    }

    let mut bytes = [0, 0];
    let reg_dst = get_valid_reg(&args[0], line_number); //Try to get valid reg

    // Determine kind of operation
    if args[1].starts_with("#") {
        // Operation is Move Immediate
        let opcode: u8 = 0x00;
        let constant: u8 = get_valid_imm(&args[1], line_number);
        bytes = encode(InstructionType::T1(opcode, reg_dst, constant));
    } else if let Some(val) = REGISTERS.get(&args[1].as_str()) {
        // Operation is Move with Registers
        let opcode: u8 = 0x01;
        let reg_src: u8 = *val;
        bytes = encode(InstructionType::T2(opcode, reg_dst, reg_src, UNUSED));
    } else {
        // Unrecognized second argument
        error_handler(LineError::Unrecognized(args[1].clone()), line_number)
    }
    bytes
}