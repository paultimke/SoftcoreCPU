use std::collections::HashMap;
use once_cell::sync::Lazy;
use crate::symbols::Symbols;
use crate::err_handler::*;

// ********************* VARIABLES AND TYPE DEFINITIONS ******************** //

// Instruction Type: Categorizes the types of instructions by the number and 
// bit sizes of the fields of the instruction.
// This Enum must be passed as parameter to the encode function to determine
// how the instruction will be encoded.
// Please refer to the CPU or Assembler Reference Manuals
enum InstructionType {
    T1(u8, u8, u8), 
    T2(u8, u8, u8, u8), 
    T3(u8, u16), 
    T4(u8, u8, u8, u8), // Fifth field unused
    T5(u8, u8, u8, u8)  // Fifth field unused
}
const UNUSED: u8 = 0x00; // Default for the Unused field in InstructionType

// Callback function returned to the parser on mnemonic matches to encode
// complete instruction as a byte pair
type EncodeCallback = fn(Vec<String>, &Symbols, usize) -> Result<[u8; 2], LineError>;

// Look-up table for Mnemonics and corresponding encoder function
pub static MNEMONICS: Lazy<HashMap<&str, EncodeCallback>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("mov", mov as EncodeCallback);
    m.insert("lda", mov as EncodeCallback);
    m.insert("ldr", mov as EncodeCallback);
    m.insert("stra", mov as EncodeCallback);
    m.insert("strr", mov as EncodeCallback);
    m.insert("push", mov as EncodeCallback);
    m.insert("pop", mov as EncodeCallback);
    m.insert("add", mov as EncodeCallback);
    m.insert("sub", mov as EncodeCallback);
    m.insert("shl", mov as EncodeCallback);
    m.insert("shr", mov as EncodeCallback);
    m.insert("and", mov as EncodeCallback);
    m.insert("or", mov as EncodeCallback);
    m.insert("not", mov as EncodeCallback);
    m.insert("jmp", mov as EncodeCallback);
    m.insert("bln", mov as EncodeCallback);
    m.insert("ret", mov as EncodeCallback);
    m.insert("cmp", mov as EncodeCallback);
    m.insert("beq", mov as EncodeCallback);
    m.insert("bne", mov as EncodeCallback);
    m.insert("bgt", mov as EncodeCallback);
    m.insert("bgtu", mov as EncodeCallback);
    m.insert("blt", mov as EncodeCallback);
    m.insert("bltu", mov as EncodeCallback);
    m.insert("halt", mov as EncodeCallback);
    m
});

// Look-up table for existing Registers and their addresses
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
    let opcode_shift = 11;
    let reg_pos0_shift = 8;
    let reg_pos1_shift = 5;
    match instr {
        InstructionType::T1(op,r,c) => {
            [(op << opcode_shift) | (r << reg_pos0_shift), c]
        }
        InstructionType::T2(op,rp0,rp1,f1) => {
            let msb = (op << opcode_shift) | (rp0 << reg_pos0_shift);
            let lsb = (rp1 << reg_pos1_shift) | f1;
            [msb, lsb]
        }
        InstructionType::T3(op,f2) => {
            let instr16bit: u16 = ((op << opcode_shift) as u16) | f2;
            let msb = (instr16bit >> 8) as u8;
            let lsb = instr16bit as u8;
            [msb, lsb]
        }
        InstructionType::T4(op,rp0,rp1,rp2) => {
            let reg_pos2_shift = 3;
            let msb = (op << opcode_shift) | (rp0 << reg_pos0_shift);
            let lsb = (rp1 << reg_pos1_shift) | (rp2 << reg_pos2_shift);
            [msb, lsb]
        }
        InstructionType::T5(op,rp0,rp1,c) => {
            let imm_shift = 1;
            let msb = (op << opcode_shift) | (rp0 << reg_pos0_shift);
            let lsb = (rp1 << reg_pos1_shift) | (c << imm_shift);
            [msb, lsb]
        }
    }
}

// Check if a given string matches the name of a valid register
// in the REGISTER look-up table
fn get_valid_reg(r: &str, line_num: usize) -> Result<u8, LineError> {
    match REGISTERS.get(r) {
        Some(val) => Ok(*val),
        None => Err(LineError::Unrecognized(String::from(r), line_num))
    }
}

// Check if a given string can be converted to a valid integer
fn get_valid_imm(c: &str, line_num: usize) -> Result<u8, LineError> {
    if c.starts_with("#") {
        let imm = match c[1..].parse::<i8>() {
            Ok(val) => Ok(val),
            Err(_) => Err(LineError::Unrecognized(String::from(c), line_num))
        }? as u8;
        return Ok(imm);
    } else {
        Err(LineError::StartWithHash(line_num))
    }
}

// Check if a given label exists as valid in symbol table
fn get_valid_label(label: &str, syms: &Symbols, line_num: usize) 
-> Result<u16, LineError> {
    match syms.labels.get(label) {
        Some(l) => Ok(*l),
        None => Err(LineError::Unrecognized(String::from(label), line_num))
    }
}

// Calls error handler in incorrect number of arguments for a given operation, 
// where f can be a function or closure that returns true is the length is valid
// or false if the length is invalid
fn check_args_len(f: impl Fn() -> bool, op: &str, line_num: usize) 
-> Result<(), LineError> {
    if !f() {
        Err(LineError::WrongArgs(op.to_string(), line_num))
    } else {
        Ok(())
    }
}

// ********************* MNEMONIC ASSEMBLING FUNCTIONS ********************* //

// MOV Operation can be one of two variants: Immediate or with Registers
// The kind of variant is determined here by the type of the second argument
// Immediate variant is of T1 and Register variant is of T2 with f1: Unused
pub fn mov(args: Vec<String>, _ : &Symbols, line_num: usize) -> 
Result<[u8; 2], LineError> {
    check_args_len(|| args.len() == 2, "mov", line_num)?;

    let reg_dst = get_valid_reg(&args[0], line_num)?; 
    
    // Determine kind of operation
    if args[1].starts_with("#") {
        // Operation is Move Immediate
        let opcode: u8 = 0x00;
        let constant: u8 = get_valid_imm(&args[1], line_num)?;
        Ok(encode(InstructionType::T1(opcode, reg_dst, constant)))
    } else if let Some(val) = REGISTERS.get(&args[1].as_str()) {
        // Operation is Move with Registers
        let opcode: u8 = 0x01;
        let reg_src: u8 = *val;
        Ok(encode(InstructionType::T2(opcode, reg_dst, reg_src, UNUSED)))
    } else {
        // Unrecognized second argument
        Err(LineError::Unrecognized(args[1].clone(), line_num))
    }
}

// LDA: Instruction belongs to T3
pub fn lda(args: Vec<String>, syms: &Symbols, line_num: usize) 
-> Result<[u8; 2], LineError> {
    check_args_len(|| args.len() == 1, "lda", line_num)?;

    let opcode = 0x02;
    let label = get_valid_label(&args[0], &syms, line_num)?;
    Ok(encode(InstructionType::T3(opcode, label)))
} 

// LDR: Instruction belongs to T2 where f1 is type Offset
pub fn ldr(args: Vec<String>, _ : &Symbols, line_num: usize) 
-> Result<[u8; 2], LineError> {
    check_args_len(|| args.len() == 2 || args.len() == 3 , "ldr", line_num)?;

    let mut offset: u8 = 0; // Default offset to 0
    let reg_dst = get_valid_reg(&args[0], line_num)?;
    if args[1].starts_with("&") {
        // Eliminate the & sign before checking if reg name is valid
        let reg_adr = get_valid_reg(&args[1][1..], line_num)?;  
         
        if args.len() == 3 {
            offset = get_valid_imm(&args[2], line_num)?;
        }

        let opcode = 0x03;
        Ok(encode(InstructionType::T2(opcode, reg_dst, reg_adr, offset)))
        
    } else {
        Err(LineError::StartWithAmp(line_num))
    }
}

// STRA: Instruction belongs to T3
pub fn stra(args: Vec<String>, syms: &Symbols, line_num: usize) 
-> Result<[u8; 2], LineError> {
    check_args_len(|| args.len() == 1, "stra", line_num)?;

    let opcode = 0x04;
    let label = get_valid_label(&args[0], &syms, line_num)?;
    Ok(encode(InstructionType::T3(opcode, label)))
}

// STRR: Instruction belongs to T2 where f1 is type Offset
pub fn strr(args: Vec<String>, _ : &Symbols, line_num: usize) 
-> Result<[u8; 2], LineError> {
    check_args_len(|| args.len() == 2 || args.len() == 3 , "strr", line_num)?;

    let mut offset: u8 = 0; // Default offset to 0
    let reg_dst = get_valid_reg(&args[0], line_num)?;
    if args[1].starts_with("&") {
        // Eliminate the & sign before checking if reg name is valid
        let reg_adr = get_valid_reg(&args[1][1..], line_num)?;  
         
        if args.len() == 3 {
            if args[2].starts_with("#") {
                offset = get_valid_imm(&args[2], line_num)?;
            } 
        }

        let opcode = 0x05;
        Ok(encode(InstructionType::T2(opcode, reg_dst, reg_adr, offset)))
        
    } else {
        Err(LineError::StartWithAmp(line_num))
    }
}

// PUSH: Instruction belongs to T4 
pub fn push(args: Vec<String>, _ : &Symbols, line_num: usize) 
-> Result<[u8; 2], LineError> {
    check_args_len(|| args.len() >=1 && args.len() <= 3 ,"push/pop operation", 
                                                                line_num)?;
    let opcode = 0x06;
    let reg_a = get_valid_reg(&args[0], line_num)?; // Reg A mandatory
    let mut reg_b = 0; // Reg B defaults to 0
    let mut reg_c = 0; // Reg C defaults to 0

    match args.len() {
        2 => reg_b = get_valid_reg(&args[1], line_num)?,
        3 => {
            reg_b = get_valid_reg(&args[1], line_num)?;
            reg_c = get_valid_reg(&args[2], line_num)?;
        }
        _ => ()
    }
    Ok(encode(InstructionType::T4(opcode, reg_a, reg_b, reg_c)))
}

// POP: Instruction belongs to T4 
// Pop instruction does exactly the same as Push instruction. They only differ
// by the opcode. Here, we implement by summing 1 to the opcode field of the
// result given by the encoding of the push instruction
pub fn pop(args: Vec<String>, _ : &Symbols, line_num: usize) 
-> Result<[u8; 2], LineError> {
    let mut bytes = push(args, &Symbols::new(), line_num)?;
    bytes[0] |= 0b0000_1000; // Sum 1 (00110 becomes 00111)
    Ok(bytes)
}

// ADD Operation can be one of two variants: Immediate or with Registers
// The kind of variant is determined here by the type of the third argument
// Immediate variant is of T2 (f: Constant) and Register variant is of T4
pub fn add(args: Vec<String>, _ : &Symbols, line_num: usize) 
-> Result<[u8; 2], LineError> {
    check_args_len(|| args.len() == 3, "add", line_num)?;

    let reg_dst = get_valid_reg(&args[0], line_num)?; 
    let reg_a = get_valid_reg(&args[1], line_num)?;

    // Determine kind of operation
    if args[2].starts_with("#") {
        // Operation is Add Immediate
        let opcode: u8 = 0x08;
        let constant: u8 = get_valid_imm(&args[2], line_num)?;
        Ok(encode(InstructionType::T2(opcode, reg_dst, reg_a, constant)))
    } else if let Some(val) = REGISTERS.get(&args[2].as_str()) {
        // Operation is Move with Registers
        let opcode: u8 = 0x09;
        let reg_b: u8 = *val;
        Ok(encode(InstructionType::T4(opcode, reg_dst, reg_a, reg_b)))
    } else {
        // Unrecognized third argument
        Err(LineError::Unrecognized(args[2].clone(), line_num))
    }
}

// SUB Operation can be one of two variants: Immediate or with Registers
// The kind of variant is determined here by the type of the third argument
// Immediate variant is of T2 (f: Constant) and Register variant is of T4
pub fn sub(args: Vec<String>, _ : &Symbols, line_num: usize) 
-> Result<[u8; 2], LineError> {
    check_args_len(|| args.len() == 3, "sub", line_num)?;

    let reg_dst = get_valid_reg(&args[0], line_num)?; 
    let reg_a = get_valid_reg(&args[1], line_num)?;

    // Determine kind of operation
    if args[2].starts_with("#") {
        // Operation is Add Immediate
        let opcode: u8 = 0x0A;
        let constant: u8 = get_valid_imm(&args[2], line_num)?;
        Ok(encode(InstructionType::T2(opcode, reg_dst, reg_a, constant)))
    } else if let Some(val) = REGISTERS.get(&args[2].as_str()) {
        // Operation is Move with Registers
        let opcode: u8 = 0x0B;
        let reg_b: u8 = *val;
        Ok(encode(InstructionType::T4(opcode, reg_dst, reg_a, reg_b)))
    } else {
        // Unrecognized third argument
        Err(LineError::Unrecognized(args[2].clone(), line_num))
    }
}

// SHL: Instruction of type T5
pub fn shl(args: Vec<String>, _ : &Symbols, line_num: usize) 
-> Result<[u8; 2], LineError> {
    check_args_len(|| args.len() == 3, "shift", line_num)?;

    let reg_dst = get_valid_reg(&args[0], line_num)?; 
    let reg_src = get_valid_reg(&args[1], line_num)?;

    if args[2].starts_with("#") {
        // Operation is Add Immediate
        let opcode: u8 = 0x0C;
        let constant: u8 = get_valid_imm(&args[2], line_num)?;
        Ok(encode(InstructionType::T5(opcode, reg_dst, reg_src, constant)))
    } else {
        // Immediate does not start with hash sign
        Err(LineError::StartWithHash(line_num))
    }
}

// SHR: Instruction belongs to T5
// SHR instruction can be implemented the same as SHR instruction. They only 
// differ by the opcode. Here, we implement by summing 1 to the opcode field of the
// result given by the encoding of the shr instruction
pub fn shr(args: Vec<String>, _ : &Symbols, line_num: usize) 
-> Result<[u8; 2], LineError> {
    let mut bytes = shl(args, &Symbols::new(), line_num)?;
    bytes[0] |= 0b0000_1000; // Sum 1 (01100 becomes 01101)
    Ok(bytes)
}

// AND: Instruction belongs to T4 
pub fn and(args: Vec<String>, _ : &Symbols, line_num: usize) 
-> Result<[u8; 2], LineError> {
    check_args_len(|| args.len() == 3 ,"logical operation", line_num)?;

    let opcode = 0x0E;
    let reg_dst = get_valid_reg(&args[0], line_num)?;
    let reg_a = get_valid_reg(&args[1], line_num)?; 
    let reg_b = get_valid_reg(&args[2], line_num)?;
    
    Ok(encode(InstructionType::T4(opcode, reg_dst, reg_a, reg_b)))
}

// OR: Instruction belongs to T4. Implementede by adding one to the opcode
// field of the encoding result of AND operation
pub fn or(args: Vec<String>, _ : &Symbols, line_num: usize) 
-> Result<[u8; 2], LineError> {
    let mut bytes = and(args, &Symbols::new(), line_num)?;
    bytes[0] |= 0b0000_1000; // Sum 1 (01110 becomes 01111)
    Ok(bytes)
}

// NOT: Instruction belongs to T2
pub fn not(args: Vec<String>, _ : &Symbols, line_num: usize) 
-> Result<[u8; 2], LineError> {
    check_args_len(|| args.len() == 2, "not", line_num)?;

    let opcode = 0x10;
    let reg_dst = get_valid_reg(&args[0], line_num)?;
    let reg_src = get_valid_reg(&args[1], line_num)?;

    Ok(encode(InstructionType::T2(opcode, reg_dst, reg_src, UNUSED)))
}