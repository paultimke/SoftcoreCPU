use std::collections::HashMap;
use once_cell::sync::Lazy;
use crate::symbols::Symbols;
use crate::err_handler::LineError;

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
const BYTE: u8 = 8;

// Callback function returned to the parser on mnemonic matches to encode
// complete instruction as a byte pair
type EncodeCallback = fn(Vec<String>, &Symbols, usize) -> Result<[u8; 2], LineError>;

// Look-up table for Mnemonics and corresponding encoder function
pub static MNEMONICS: Lazy<HashMap<&str, EncodeCallback>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("mov", mov as EncodeCallback);
    m.insert("lda", lda as EncodeCallback);
    m.insert("ldr", ldr as EncodeCallback);
    m.insert("stra", stra as EncodeCallback);
    m.insert("strr", strr as EncodeCallback);
    m.insert("push", push as EncodeCallback);
    m.insert("pop", pop as EncodeCallback);
    m.insert("add", add as EncodeCallback);
    m.insert("sub", sub as EncodeCallback);
    m.insert("shl", shl as EncodeCallback);
    m.insert("shr", shr as EncodeCallback);
    m.insert("and", and as EncodeCallback);
    m.insert("or", or as EncodeCallback);
    m.insert("not", not as EncodeCallback);
    m.insert("jmp", jmp as EncodeCallback);
    m.insert("bln", bln as EncodeCallback);
    m.insert("ret", ret as EncodeCallback);
    m.insert("cmp", cmp as EncodeCallback);
    m.insert("beq", beq as EncodeCallback);
    m.insert("bne", bne as EncodeCallback);
    m.insert("bgt", bgt as EncodeCallback);
    m.insert("bgtu", bgtu as EncodeCallback);
    m.insert("blt", blt as EncodeCallback);
    m.insert("bltu", bltu as EncodeCallback);
    m.insert("halt", halt as EncodeCallback);
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
    let opcode_shift  = 11;
    let reg_pos0_shift = 8;
    let reg_pos1_shift = 5;
    match instr {
        InstructionType::T1(op,r,c) => {
            // TODO: Fix shift overflow. Must treat vars first as u16 then cast
            let msb = {
                let x = ((op as u16) << opcode_shift) | ((r as u16) << reg_pos0_shift);
                (x >> BYTE) as u8
            };
            [msb, c]
        }
        InstructionType::T2(op,rp0,rp1,f1) => {
            let msb = {
                let x = ((op as u16) << opcode_shift) | ((rp0 as u16) << reg_pos0_shift); 
                (x >> BYTE) as u8
            };
            let lsb = (((rp1 as u16) << reg_pos1_shift) | (f1 as u16)) as u8;
            [msb, lsb]
        }
        InstructionType::T3(op,f2) => {
            let instr16bit: u16 = ((op as u16) << opcode_shift) | f2 ;
            let msb = (instr16bit >> 8) as u8;
            let lsb = instr16bit as u8;
            [msb, lsb]
        }
        InstructionType::T4(op,rp0,rp1,rp2) => {
            let reg_pos2_shift = 2;
            let msb = {
                let x = ((op as u16) << opcode_shift) | ((rp0 as u16) << reg_pos0_shift);
                (x >> BYTE) as u8 
            };
            let lsb = ((rp1 as u16) << reg_pos1_shift) | ((rp2 as u16) << reg_pos2_shift);
            [msb, lsb as u8]
        }
        InstructionType::T5(op,rp0,rp1,c) => {
            let imm_shift = 1;
            let msb = {
                let x = ((op as u16) << opcode_shift) | ((rp0 as u16) << reg_pos0_shift);
                (x >> BYTE) as u8 
            };
            let lsb = (((rp1 as u16) << reg_pos1_shift) | ((c as u16) << imm_shift)) as u8;
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

// JMP: Instruction belongs to T3
pub fn jmp(args: Vec<String>, syms : &Symbols, line_num: usize) 
-> Result<[u8; 2], LineError> {
    check_args_len(|| args.len() == 1, "jmp", line_num)?;
    let opcode = 0x11;
    let label = get_valid_label(&args[0], syms, line_num)?;
    Ok(encode(InstructionType::T3(opcode, label)))
}

// BLN: Instruction belongs to T3
pub fn bln(args: Vec<String>, syms : &Symbols, line_num: usize) 
-> Result<[u8; 2], LineError> {
    check_args_len(|| args.len() == 1, "bln", line_num)?;
    let opcode = 0x12;
    let label = get_valid_label(&args[0], syms, line_num)?;
    Ok(encode(InstructionType::T3(opcode, label)))
}

// RET: Instruction belongs to T3
pub fn ret(args: Vec<String>, _ : &Symbols, line_num: usize) 
-> Result<[u8; 2], LineError> {
    check_args_len(|| args.len() == 0, "ret", line_num)?;
    let opcode = 0x13;
    Ok(encode(InstructionType::T3(opcode, UNUSED as u16)))
}

// CMP: Instruction may be one of two variants.
// Immediate variant is of T1 and Register variant of T2 (f1: Unused)
pub fn cmp(args: Vec<String>, _ : &Symbols, line_num: usize) 
-> Result<[u8; 2], LineError> {
    check_args_len(|| args.len() == 2, "cmp", line_num)?;

    let reg_a = get_valid_reg(&args[0], line_num)?; 
    
    // Determine kind of operation
    if args[1].starts_with("#") {
        // Operation is Move Immediate
        let opcode: u8 = 0x14;
        let constant: u8 = get_valid_imm(&args[1], line_num)?;
        Ok(encode(InstructionType::T1(opcode, reg_a, constant)))
    } else if let Some(val) = REGISTERS.get(&args[1].as_str()) {
        // Operation is Move with Registers
        let opcode: u8 = 0x15;
        let reg_b: u8 = *val;
        Ok(encode(InstructionType::T2(opcode, reg_a, reg_b, UNUSED)))
    } else {
        // Unrecognized second argument
        Err(LineError::Unrecognized(args[1].clone(), line_num))
    }
}

// BEQ: Instruction belongs to T3
pub fn beq(args: Vec<String>, syms : &Symbols, line_num: usize) 
-> Result<[u8; 2], LineError> {
    check_args_len(|| args.len() == 1, "beq", line_num)?;
    let opcode = 0x16;
    let label = get_valid_label(&args[0], syms, line_num)?;
    Ok(encode(InstructionType::T3(opcode, label)))
}

// BNE: Instruction belongs to T3
pub fn bne(args: Vec<String>, syms : &Symbols, line_num: usize) 
-> Result<[u8; 2], LineError> {
    check_args_len(|| args.len() == 1, "bne", line_num)?;
    let opcode = 0x17;
    let label = get_valid_label(&args[0], syms, line_num)?;
    Ok(encode(InstructionType::T3(opcode, label)))
}

// BGT: Instruction belongs to T3
pub fn bgt(args: Vec<String>, syms : &Symbols, line_num: usize) 
-> Result<[u8; 2], LineError> {
    check_args_len(|| args.len() == 1, "bgt", line_num)?;
    let opcode = 0x18;
    let label = get_valid_label(&args[0], syms, line_num)?;
    Ok(encode(InstructionType::T3(opcode, label)))
}

// BGTU: Instruction belongs to T3
pub fn bgtu(args: Vec<String>, syms : &Symbols, line_num: usize) 
-> Result<[u8; 2], LineError> {
    check_args_len(|| args.len() == 1, "bgtu", line_num)?;
    let opcode = 0x19;
    let label = get_valid_label(&args[0], syms, line_num)?;
    Ok(encode(InstructionType::T3(opcode, label)))
}

// BLT: Instruction belongs to T3
pub fn blt(args: Vec<String>, syms : &Symbols, line_num: usize) 
-> Result<[u8; 2], LineError> {
    check_args_len(|| args.len() == 1, "blt", line_num)?;
    let opcode = 0x1A;
    let label = get_valid_label(&args[0], syms, line_num)?;
    Ok(encode(InstructionType::T3(opcode, label)))
}

// BLTU: Instruction belongs to T3
pub fn bltu(args: Vec<String>, syms : &Symbols, line_num: usize) 
-> Result<[u8; 2], LineError> {
    check_args_len(|| args.len() == 1, "bltu", line_num)?;
    let opcode = 0x1B;
    let label = get_valid_label(&args[0], syms, line_num)?;
    Ok(encode(InstructionType::T3(opcode, label)))
}

// HALT: Instruction belongs to T3
pub fn halt(args: Vec<String>, _ : &Symbols, line_num: usize) 
-> Result<[u8; 2], LineError> {
    check_args_len(|| args.len() == 0, "halt", line_num)?;
    let opcode = 0x1C;
    Ok(encode(InstructionType::T3(opcode, UNUSED as u16)))
}