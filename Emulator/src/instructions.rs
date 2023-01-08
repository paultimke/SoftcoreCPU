// Libraries
use num_derive::FromPrimitive;

// Type Declarations
type Memory = Vec<u16>;

// Opcode numerical translations
#[derive(FromPrimitive)]
pub enum Opcode {
    MovIm,  // MOV immediate (MOV {reg_dst} {constant})
    MovRg,  // MOV with registers (MOV {reg_dst} {reg_src})
    Load,   // LOAD immediate (LOAD {label})
    LoadRg, // LOAD with registers (LOAD {reg_dst} {reg_adr} {offset})
    Store,  // STR immediate (STR {label})
    StrRg,  // STR with registers (STR {reg_src} {reg_adr} {offset})
    Push,   // PUSH (PUSH {reg_A} {reg_B} {reg_C} {num})
    Pop,    // POP (POP {reg_A} {reg_B} {reg_C} {num})
    AddIm,  // ADD immediate (ADD {reg_dst} {reg_A} {constant})
    AddRg,  // ADD with registers (ADD {reg_dst} {reg_A} {reg_B})
    SubIm,  // SUB immediate (SUB {reg_dst} {reg_A} {constant})
    SubRg,  // SUB with registers (SUB {reg_dst} {reg_A} {reg_B})
    ShftL,  // Shift Left (SHL {reg_dst} {reg_src} {constant})
    ShftR,  // Shift Right (SHR {reg_dst} {reg_src} {constant})
    And,    // Biwise AND (AND {reg_dst} {reg_A} {reg_B})
    Or,     // Bitwise OR (OR {reg_dst} {reg_A} {reg_B})
    Not,    // Bitwise NOT (NOT {reg_dst} {reg_src})
    Bal,    // Branch always (BAL {label})
    BZR,    // Branch if zero (BZR {label})
    BNG,    // Branch if negative (BNG {label})
    BLN,    // Branch with link (BLN {label})
    RET,    // Return from branch (RET)
    Halt,   // Halts program execution until system reset
}

// Instruction frame constants
// SIZE endings:
//     refer to the size of the bitfield
// ROFFSET endings:
//     refer to the amount of bits from the right until bitfield begins
// Example: 0110 01[00] 0011 1101 has SIZE = 2 and ROFFSET = 6
const OPCODE_SIZE: usize          = 5;
const REG_ADDR_SIZE: usize        = 3;
const REG_POS0_ROFFSET: usize     = OPCODE_SIZE;
const REG_POS1_ROFFSET: usize     = REG_POS0_ROFFSET + REG_ADDR_SIZE;
const REG_POS2_ROFFSET: usize     = REG_POS1_ROFFSET + REG_ADDR_SIZE;
const MEM_OFFSET_SIZE: usize      = 5;
const MEM_OFFSET_ROFFSET: usize   = REG_POS2_ROFFSET;
const MEM_LABEL_SIZE: usize       = 11;
const MEM_LABEL_ROFFSET: usize    = OPCODE_SIZE;
const PUSHPOP_NUM_SIZE: usize     = 2;
const PUSHPOP_NUM_ROFFSET: usize  = REG_POS2_ROFFSET + REG_ADDR_SIZE;
const MOV_CONSTANT_SIZE: usize    = 8;
const MOV_CONSTANT_ROFFSET: usize = REG_POS0_ROFFSET + REG_ADDR_SIZE;
const MATH_CONSTANT_SIZE: usize   = 5;
const MATH_CONSTANT_ROFFSET:usize = REG_POS1_ROFFSET + REG_ADDR_SIZE;
const SHFT_CONSTANT_SIZE: usize   = 4;
const SHFT_CONSTANT_ROFFSET: usize = REG_POS1_ROFFSET + REG_ADDR_SIZE;

// Module to execute instructions
pub mod execute {
    use super::{Memory, extract_bits, REG_ADDR_SIZE, REG_POS0_ROFFSET, 
                MOV_CONSTANT_SIZE, MOV_CONSTANT_ROFFSET, REG_POS1_ROFFSET,
                REG_POS2_ROFFSET, MEM_LABEL_SIZE, MEM_LABEL_ROFFSET, 
                MEM_OFFSET_SIZE, MEM_OFFSET_ROFFSET, PUSHPOP_NUM_SIZE, 
                PUSHPOP_NUM_ROFFSET, MATH_CONSTANT_ROFFSET, MATH_CONSTANT_SIZE, 
                SHFT_CONSTANT_SIZE, SHFT_CONSTANT_ROFFSET,
    };
    use super::super::registers::{Registers, Flags, MBR_PTR, SP_PTR};

    // Moves an immediate constant value into a destination register
    pub fn mov_im(regs: &mut Registers) -> () {
        let reg_dst = extract_bits(regs.ir, REG_ADDR_SIZE, REG_POS0_ROFFSET);
        let val = extract_bits(regs.ir, MOV_CONSTANT_SIZE, MOV_CONSTANT_ROFFSET);
        regs.gp[reg_dst] = val as i16;
    }

    // Moves the value of a source register into destination register
    pub fn mov_rg(regs: &mut Registers) -> () {
        let reg_dst = extract_bits(regs.ir, REG_ADDR_SIZE, REG_POS0_ROFFSET);
        let reg_src = extract_bits(regs.ir, REG_ADDR_SIZE, REG_POS1_ROFFSET);
        regs.gp[reg_dst] = regs.gp[reg_src];
    }

    // Loads contents from memory at address label into Memory Buffer Register
    pub fn load(regs: &mut Registers, mem: &Memory) -> () {
        let label = extract_bits(regs.ir, MEM_LABEL_SIZE, MEM_LABEL_ROFFSET); 
        regs.gp[MBR_PTR] = mem[label] as i16;
    }

    // Loads contents of memory at address specified by reg_adr 
    // into destination register
    pub fn load_rg(regs: &mut Registers, mem: &Memory) -> () {
        let reg_dst = extract_bits(regs.ir, REG_ADDR_SIZE, REG_POS0_ROFFSET);
        let reg_adr = extract_bits(regs.ir, REG_ADDR_SIZE, REG_POS1_ROFFSET); 
        let ofst = extract_bits(regs.ir, MEM_OFFSET_SIZE, MEM_OFFSET_ROFFSET);
        regs.gp[reg_dst] = mem[(reg_adr + ofst)] as i16;
    }

    // Stores contents from Memory Buffer Register into memory at address label
    pub fn store(regs: &Registers, mem: &mut Memory) -> () {
        let label = extract_bits(regs.ir, MEM_LABEL_SIZE, MEM_LABEL_ROFFSET); 
        mem[label] = regs.gp[MBR_PTR] as u16;
    }
    
    // Stores contents from Source Register into memory at address 
    // specified by reg_adr 
    pub fn store_rg(regs: &Registers, mem: &mut Memory) -> () {
        let reg_src = extract_bits(regs.ir, REG_ADDR_SIZE, REG_POS0_ROFFSET);
        let reg_adr = extract_bits(regs.ir, REG_ADDR_SIZE, REG_POS1_ROFFSET); 
        let ofst = extract_bits(regs.ir, MEM_OFFSET_SIZE, MEM_OFFSET_ROFFSET);
        mem[(reg_adr + ofst)] = regs.gp[reg_src] as u16;
    }

    // Push up to three different registers onto the stack
    pub fn push(regs: &mut Registers, mem: &mut Memory) -> () {
        let num = extract_bits(regs.ir, PUSHPOP_NUM_SIZE, PUSHPOP_NUM_ROFFSET);
        for i in 0..num {
            // Get address of each register
            let addr = extract_bits(regs.ir, 
                                    REG_ADDR_SIZE, 
                                    REG_POS0_ROFFSET + (REG_ADDR_SIZE*(i as usize)));

            regs.gp[SP_PTR] -= 1; // Update top
            mem[regs.gp[SP_PTR] as usize] = regs.gp[addr] as u16; // Push to stack
        }
    }

    // Pop up to three different registers from the stack and into the registers
    pub fn pop(regs: &mut Registers, mem: &Memory) -> () {
        let num = extract_bits(regs.ir, PUSHPOP_NUM_SIZE, PUSHPOP_NUM_ROFFSET);
        for i in 0..num {
            // Get address of each register
            let addr = extract_bits(regs.ir, 
                                    REG_ADDR_SIZE, 
                                    REG_POS0_ROFFSET + (REG_ADDR_SIZE*(i as usize)));

            regs.gp[addr] = mem[regs.gp[SP_PTR] as usize] as i16; // Pop to register
            regs.gp[SP_PTR] += 1; // Update top
        }
    }

    // Adds an immediate constant to reg_A and stores the result in reg_dst
    pub fn add_im(regs: &mut Registers) -> () {
        let reg_dst = extract_bits(regs.ir, REG_ADDR_SIZE, REG_POS0_ROFFSET);
        let reg_a = extract_bits(regs.ir, REG_ADDR_SIZE, REG_POS1_ROFFSET);
        let val = extract_bits(regs.ir, MATH_CONSTANT_SIZE, MATH_CONSTANT_ROFFSET);

        match regs.gp[reg_a].checked_add(val as i16) {
            Some(v) => {
                regs.gp[reg_dst] = v;
                regs.change_flags(vec![Flags::OV(false), Flags::CA(false)]);
                match v {
                    0          => regs.change_flags(vec![Flags::ZR(true),
                                                         Flags::NG(false)]),
                    v if v < 0 => regs.change_flags(vec![Flags::ZR(false),
                                                         Flags::NG(true)]),
                    _          => (),
                }
            }
            None => regs.change_flags(vec![Flags::OV(true), Flags::CA(true)])
        }
    }

    // Adds the contents of reg_A and reg_B and stores the result in reg_dst
    pub fn add_rg(regs: &mut Registers) -> () {
        let reg_dst = extract_bits(regs.ir, REG_ADDR_SIZE, REG_POS0_ROFFSET);
        let reg_a = extract_bits(regs.ir, REG_ADDR_SIZE, REG_POS1_ROFFSET);
        let reg_b = extract_bits(regs.ir, REG_ADDR_SIZE, REG_POS2_ROFFSET);

        match regs.gp[reg_a].checked_add(regs.gp[reg_b]) {
            Some(v) => {
                regs.gp[reg_dst] = v;
                regs.change_flags(vec![Flags::OV(false), Flags::CA(false)]);
                match v {
                    0          => regs.change_flags(vec![Flags::ZR(true),
                                                         Flags::NG(false)]),
                    v if v < 0 => regs.change_flags(vec![Flags::ZR(false),
                                                         Flags::NG(true)]),
                    _          => (),
                }
            }
            None => regs.change_flags(vec![Flags::OV(true), Flags::CA(true)])
        }
    }

    // Subtracts an immediate constant to reg_A and stores the result in reg_dst
    pub fn sub_im(regs: &mut Registers) -> () {
        let reg_dst = extract_bits(regs.ir, REG_ADDR_SIZE, REG_POS0_ROFFSET);
        let reg_a = extract_bits(regs.ir, REG_ADDR_SIZE, REG_POS1_ROFFSET);
        let val = extract_bits(regs.ir, MATH_CONSTANT_SIZE, MATH_CONSTANT_ROFFSET);

        match regs.gp[reg_a].checked_sub(val as i16) {
            Some(v) => {
                regs.gp[reg_dst] = v;
                regs.change_flags(vec![Flags::OV(false)]);
                match v {
                    v if v == 0 => regs.change_flags(vec![Flags::CA(true),
                                                          Flags::ZR(true),
                                                          Flags::NG(false)]),
                    v if v > 0  => regs.change_flags(vec![Flags::CA(true),
                                                          Flags::ZR(false),
                                                          Flags::NG(false)]),
                    _           => regs.change_flags(vec![Flags::CA(false),
                                                          Flags::ZR(false),
                                                          Flags::NG(true)]),
                }
            }
            None => regs.change_flags(vec![Flags::OV(true)])
        }
    }

    // Subtracts the contents of reg_A and reg_B and stores the result in reg_dst
    pub fn sub_rg(regs: &mut Registers) -> () {
        let reg_dst = extract_bits(regs.ir, REG_ADDR_SIZE, REG_POS0_ROFFSET);
        let reg_a = extract_bits(regs.ir, REG_ADDR_SIZE, REG_POS1_ROFFSET);
        let reg_b = extract_bits(regs.ir, REG_ADDR_SIZE, REG_POS2_ROFFSET);

        match regs.gp[reg_a].checked_sub(regs.gp[reg_b]) {
            Some(v) => {
                regs.gp[reg_dst] = v;
                regs.change_flags(vec![Flags::OV(false)]);
                match v {
                    v if v == 0 => regs.change_flags(vec![Flags::CA(true),
                                                          Flags::ZR(true),
                                                          Flags::NG(false)]),
                    v if v > 0  => regs.change_flags(vec![Flags::CA(true),
                                                          Flags::ZR(false),
                                                          Flags::NG(false)]),
                    _           => regs.change_flags(vec![Flags::CA(false),
                                                          Flags::ZR(false),
                                                          Flags::NG(true)]),
                }
            }
            None => regs.change_flags(vec![Flags::OV(true)])
        }
    }
    
    // Shifts constant bits left from reg_src and stores result in reg_dst
    pub fn shift_l(regs: &mut Registers) -> () {
        let reg_dst = extract_bits(regs.ir, REG_ADDR_SIZE, REG_POS0_ROFFSET);
        let reg_src = extract_bits(regs.ir, REG_ADDR_SIZE, REG_POS1_ROFFSET);
        let val = extract_bits(regs.ir, SHFT_CONSTANT_SIZE, SHFT_CONSTANT_ROFFSET);
        
        regs.gp[reg_dst] = regs.gp[reg_src] << (val as u16);
        if regs.gp[reg_dst] == 0 {
            regs.change_flags(vec![Flags::ZR(true), Flags::NG(false)]);
        }
        else if regs.gp[reg_dst] < 0 {
            regs.change_flags(vec![Flags::ZR(false), Flags::NG(true)]);
        }

        regs.change_flags(vec![Flags::OV(false), Flags::CA(false)]);
    }

    // Shifts constant bits left from reg_src and stores result in reg_dst
    pub fn shift_r(regs: &mut Registers) -> () {
        let reg_dst = extract_bits(regs.ir, REG_ADDR_SIZE, REG_POS0_ROFFSET);
        let reg_src = extract_bits(regs.ir, REG_ADDR_SIZE, REG_POS1_ROFFSET);
        let val = extract_bits(regs.ir, SHFT_CONSTANT_SIZE, SHFT_CONSTANT_ROFFSET);
        
        regs.gp[reg_dst] = regs.gp[reg_src] >> (val as u16);
        if regs.gp[reg_dst] == 0 {
            regs.change_flags(vec![Flags::ZR(true), Flags::NG(false)]);
        }
        else if regs.gp[reg_dst] < 0 {
            regs.change_flags(vec![Flags::ZR(false), Flags::NG(true)]);
        }

        regs.change_flags(vec![Flags::OV(false), Flags::CA(false)]);
    }

    // Bitwise ANDs reg_A and reg_B and stores the result in reg_dst
    pub fn and(regs: &mut Registers) -> () {
        let reg_dst = extract_bits(regs.ir, REG_ADDR_SIZE, REG_POS0_ROFFSET);
        let reg_a = extract_bits(regs.ir, REG_ADDR_SIZE, REG_POS1_ROFFSET);
        let reg_b = extract_bits(regs.ir, REG_ADDR_SIZE, REG_POS2_ROFFSET);

        regs.gp[reg_dst] = regs.gp[reg_a] & regs.gp[reg_b];
        if regs.gp[reg_dst] == 0 {
            regs.change_flags(vec![Flags::ZR(true), Flags::NG(false)]);
        }
        else if regs.gp[reg_dst] < 0 {
            regs.change_flags(vec![Flags::ZR(false), Flags::NG(true)]);
        }

        regs.change_flags(vec![Flags::OV(false), Flags::CA(false)]);
    }

    // Bitwise ORs reg_A and reg_B and stores the result in reg_dst
    pub fn or(regs: &mut Registers) -> () {
        let reg_dst = extract_bits(regs.ir, REG_ADDR_SIZE, REG_POS0_ROFFSET);
        let reg_a = extract_bits(regs.ir, REG_ADDR_SIZE, REG_POS1_ROFFSET);
        let reg_b = extract_bits(regs.ir, REG_ADDR_SIZE, REG_POS2_ROFFSET);

        regs.gp[reg_dst] = regs.gp[reg_a] | regs.gp[reg_b];
        if regs.gp[reg_dst] == 0 {
            regs.change_flags(vec![Flags::ZR(true), Flags::NG(false)]);
        }
        else if regs.gp[reg_dst] < 0 {
            regs.change_flags(vec![Flags::ZR(false), Flags::NG(true)]);
        }

        regs.change_flags(vec![Flags::OV(false), Flags::CA(false)]);
    }

    // Bitwise inverts reg_src and stores the result in reg_dst
    pub fn not(regs: &mut Registers) -> () {
        let reg_dst = extract_bits(regs.ir, REG_ADDR_SIZE, REG_POS0_ROFFSET);
        let reg_src = extract_bits(regs.ir, REG_ADDR_SIZE, REG_POS1_ROFFSET);

        regs.gp[reg_dst] = !regs.gp[reg_src];
        if regs.gp[reg_dst] == 0 {
            regs.change_flags(vec![Flags::ZR(true), Flags::NG(false)]);
        }
        else if regs.gp[reg_dst] < 0 {
            regs.change_flags(vec![Flags::ZR(false), Flags::NG(true)]);
        }

        regs.change_flags(vec![Flags::OV(false), Flags::CA(false)]);
    }

    // Branch to address in label
    pub fn bal(regs: &mut Registers) -> () {
        let label = extract_bits(regs.ir, MEM_LABEL_SIZE, MEM_LABEL_ROFFSET);
        regs.pc = label as u16;
    }

    // Branch if zero 
    /*pub fn bzr(regs: &mut Registers, mem: &Memory) -> () {
        if regs.flags
        let label = extract_bits(regs.ir, MEM_LABEL_SIZE, MEM_LABEL_ROFFSET);
        regs.pc = label as u16;
    }*/
}


// Helper function to extract the value of size amount of bits offseted
// by right_offset from the right.
// Example: size = 3, right_offset = 5 on num = 0110 1[011] 1000 0110 
//          yields -> 011 (decimal 7)
pub fn extract_bits(num: u16, size: usize, right_offset: usize) -> usize {
    let left_offset = 16 - right_offset - size;
    
    // Mask to clear the bits other thant what we're interested
    let mut mask = 1; 
    for _ in 1..size {
        mask <<= 1;
        mask |= 1;
    }
    mask <<= left_offset;

    // Bitwise AND the number with the mask and undo the shift
    // to get actual value
    ((num&mask) >> left_offset) as usize
}