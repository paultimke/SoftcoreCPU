use colored::Colorize;
use std::collections::HashMap;
use once_cell::sync::Lazy;
use crate::parser::{error_handler, LineError};

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

// Encodes the passed in values by their InstructionType as specified
// by the reference manual.
fn encode(instr: InstructionType) -> [u8; 2] {
    [1u8, 2u8]
}

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

// MOV Opcode can be one of two variants: Immediate or with Registers
// The kind of variant is determined here by the type of the second argument
pub fn mov(args: Vec<String>, line_number: usize) -> [u8; 2] {
    let mut bytes = [0, 0];

    if args.len() != 2 {
        let msg = format!("{} operation can only have 2 parameters", "mov".bold());
        error_handler(LineError::WrongArgs(msg), line_number)
    }

    // Check if 1st argument (reg_dst) exists in register table
    let reg_dst = match REGISTERS.get(&args[0].as_str()) {
        Some(val) => *val,
        None => {error_handler(LineError::Unrecognized(args[0].clone()), line_number); 0}
    };

    // Determine kind of operation
    if args[1].starts_with("#") {
        // Operation is Move Immediate
        let opcode: u8 = 0x00;
        let constant: u8 = match &args[1][1..].parse::<i8>() {
            Ok(val) => *val,
            Err(_) => {error_handler(LineError::Unrecognized(args[0].clone()), line_number); 0}
        } as u8;
        bytes = encode(InstructionType::T1(opcode, reg_dst, constant));
    } else if let Some(val) = REGISTERS.get(&args[1].as_str()) {
        // Operation is Move with Registers
        let opcode: u8 = 0x01;
        let reg_src: u8 = *val;
        bytes = encode(InstructionType::T2(opcode, reg_dst, reg_src, UNUSED));
    }
    else {
        // Unrecognized second argument
        error_handler(LineError::Unrecognized(args[1].clone()), line_number)
    }
    
    bytes
}




