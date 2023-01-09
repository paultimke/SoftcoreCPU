// Libraries
use num_derive::FromPrimitive;

// Type Declarations
type Memory = Vec<u16>;

// Opcode numerical translations (From 0x00 to 0x16)
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
    Jmp,    // Branch always (Jmp {label})
    Bln,    // Branch with link (BLN {label})
    Ret,    // Return from branch (RET {label})
    CmpIm,  // Compare immediate (CMP {reg_A} {constant})
    CmpRg,  // Compare registers (CMP {reg_A} {reg_B})
    Beq,    // Branch if equal (BEQ {label})
    Bne,    // Branch if not equal (BNE {label})
    Bgt,    // Branch if greater than signed (BGT {label})
    Bgtu,   // Branch if greater than unsigned (BGTU {label})
    Blt,    // Branch if less than signed (BLT {label})
    Bltu,   // Branch if less than unsigned (BLTU {label})
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
const B8_CONSTANT_SIZE: usize     = 8;
const B8_CONSTANT_ROFFSET: usize  = REG_POS0_ROFFSET + REG_ADDR_SIZE;
const B5_CONSTANT_SIZE: usize     = 5;
const B5_CONSTANT_ROFFSET:usize   = REG_POS1_ROFFSET + REG_ADDR_SIZE;
const B4_CONSTANT_SIZE: usize     = 4;
const B4_CONSTANT_ROFFSET: usize  = REG_POS1_ROFFSET + REG_ADDR_SIZE;

// Module to execute instructions
pub mod execute {
    use super::{Memory, extract_bits, REG_ADDR_SIZE, REG_POS0_ROFFSET, 
                B8_CONSTANT_SIZE, B8_CONSTANT_ROFFSET, REG_POS1_ROFFSET,
                REG_POS2_ROFFSET, MEM_LABEL_SIZE, MEM_LABEL_ROFFSET, 
                MEM_OFFSET_SIZE, MEM_OFFSET_ROFFSET, PUSHPOP_NUM_SIZE, 
                PUSHPOP_NUM_ROFFSET, B5_CONSTANT_ROFFSET, B5_CONSTANT_SIZE, 
                B4_CONSTANT_SIZE, B4_CONSTANT_ROFFSET,
    };
    use super::super::registers::{Registers, Flags, MBR_PTR, SP_PTR, LNR_PTR};

    // Moves an immediate constant value into a destination register
    pub fn mov_im(regs: &mut Registers) -> () {
        let reg_dst = extract_bits(regs.ir, REG_ADDR_SIZE, REG_POS0_ROFFSET);
        let val = extract_bits(regs.ir, B8_CONSTANT_SIZE, B8_CONSTANT_ROFFSET);
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
        let val = extract_bits(regs.ir, B5_CONSTANT_SIZE, B5_CONSTANT_ROFFSET);

        match regs.gp[reg_a].checked_add(val as i16) {
            Some(v) => {
                regs.gp[reg_dst] = v;
                regs.change_flags(vec![(Flags::OV, false), (Flags::CA, false)]);
                match v {
                    0          => regs.change_flags(vec![(Flags::ZR, true),
                                                         (Flags::NG, false)]),
                    v if v < 0 => regs.change_flags(vec![(Flags::ZR, false),
                                                         (Flags::NG, true)]),
                    _          => (),
                }
            }
            None => regs.change_flags(vec![(Flags::OV, true), (Flags::CA, true)])
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
                regs.change_flags(vec![(Flags::OV, false), (Flags::CA, false)]);
                match v {
                    0          => regs.change_flags(vec![(Flags::ZR, true),
                                                         (Flags::NG, false)]),
                    v if v < 0 => regs.change_flags(vec![(Flags::ZR, false),
                                                         (Flags::NG, true)]),
                    _          => (),
                }
            }
            None => regs.change_flags(vec![(Flags::OV, true), (Flags::CA, true)])
        }
    }

    // Subtracts an immediate constant to reg_A and stores the result in reg_dst
    pub fn sub_im(regs: &mut Registers) -> () {
        let reg_dst = extract_bits(regs.ir, REG_ADDR_SIZE, REG_POS0_ROFFSET);
        let reg_a = extract_bits(regs.ir, REG_ADDR_SIZE, REG_POS1_ROFFSET);
        let val = extract_bits(regs.ir, B5_CONSTANT_SIZE, B5_CONSTANT_ROFFSET);

        match regs.gp[reg_a].checked_sub(val as i16) {
            Some(v) => {
                regs.gp[reg_dst] = v;
                regs.change_flags(vec![(Flags::OV, false)]);
                match v {
                    v if v == 0 => regs.change_flags(vec![(Flags::CA, true),
                                                          (Flags::ZR, true),
                                                          (Flags::NG, false)]),
                    v if v > 0  => regs.change_flags(vec![(Flags::CA, true),
                                                          (Flags::ZR, false),
                                                          (Flags::NG, false)]),
                    _           => regs.change_flags(vec![(Flags::CA, false),
                                                          (Flags::ZR, false),
                                                          (Flags::NG, true)]),
                }
            }
            None => regs.change_flags(vec![(Flags::OV, true)])
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
                regs.change_flags(vec![(Flags::OV, false)]);
                match v {
                    v if v == 0 => regs.change_flags(vec![(Flags::CA, true),
                                                          (Flags::ZR, true),
                                                          (Flags::NG, false)]),
                    v if v > 0  => regs.change_flags(vec![(Flags::CA, true),
                                                          (Flags::ZR, false),
                                                          (Flags::NG, false)]),
                    _           => regs.change_flags(vec![(Flags::CA, false),
                                                          (Flags::ZR, false),
                                                          (Flags::NG, true)]),
                }
            }
            None => regs.change_flags(vec![(Flags::OV, true)])
        }
    }
    
    // Shifts constant bits left from reg_src and stores result in reg_dst
    pub fn shift_l(regs: &mut Registers) -> () {
        let reg_dst = extract_bits(regs.ir, REG_ADDR_SIZE, REG_POS0_ROFFSET);
        let reg_src = extract_bits(regs.ir, REG_ADDR_SIZE, REG_POS1_ROFFSET);
        let val = extract_bits(regs.ir, B4_CONSTANT_SIZE, B4_CONSTANT_ROFFSET);
        
        regs.gp[reg_dst] = regs.gp[reg_src] << (val as u16);
        if regs.gp[reg_dst] == 0 {
            regs.change_flags(vec![(Flags::ZR, true), (Flags::NG, false)]);
        }
        else if regs.gp[reg_dst] < 0 {
            regs.change_flags(vec![(Flags::ZR, false), (Flags::NG, true)]);
        }

        regs.change_flags(vec![(Flags::OV, false), (Flags::CA, false)]);
    }

    // Shifts constant bits left from reg_src and stores result in reg_dst
    pub fn shift_r(regs: &mut Registers) -> () {
        let reg_dst = extract_bits(regs.ir, REG_ADDR_SIZE, REG_POS0_ROFFSET);
        let reg_src = extract_bits(regs.ir, REG_ADDR_SIZE, REG_POS1_ROFFSET);
        let val = extract_bits(regs.ir, B4_CONSTANT_SIZE, B4_CONSTANT_ROFFSET);
        
        regs.gp[reg_dst] = regs.gp[reg_src] >> (val as u16);
        if regs.gp[reg_dst] == 0 {
            regs.change_flags(vec![(Flags::ZR, true), (Flags::NG, false)]);
        }
        else if regs.gp[reg_dst] < 0 {
            regs.change_flags(vec![(Flags::ZR, false), (Flags::NG, true)]);
        }

        regs.change_flags(vec![(Flags::OV, false), (Flags::CA, false)]);
    }

    // Bitwise ANDs reg_A and reg_B and stores the result in reg_dst
    pub fn and(regs: &mut Registers) -> () {
        let reg_dst = extract_bits(regs.ir, REG_ADDR_SIZE, REG_POS0_ROFFSET);
        let reg_a = extract_bits(regs.ir, REG_ADDR_SIZE, REG_POS1_ROFFSET);
        let reg_b = extract_bits(regs.ir, REG_ADDR_SIZE, REG_POS2_ROFFSET);

        regs.gp[reg_dst] = regs.gp[reg_a] & regs.gp[reg_b];
        if regs.gp[reg_dst] == 0 {
            regs.change_flags(vec![(Flags::ZR, true), (Flags::NG, false)]);
        }
        else if regs.gp[reg_dst] < 0 {
            regs.change_flags(vec![(Flags::ZR, false), (Flags::NG, true)]);
        }

        regs.change_flags(vec![(Flags::OV, false), (Flags::CA, false)]);
    }

    // Bitwise ORs reg_A and reg_B and stores the result in reg_dst
    pub fn or(regs: &mut Registers) -> () {
        let reg_dst = extract_bits(regs.ir, REG_ADDR_SIZE, REG_POS0_ROFFSET);
        let reg_a = extract_bits(regs.ir, REG_ADDR_SIZE, REG_POS1_ROFFSET);
        let reg_b = extract_bits(regs.ir, REG_ADDR_SIZE, REG_POS2_ROFFSET);

        regs.gp[reg_dst] = regs.gp[reg_a] | regs.gp[reg_b];
        if regs.gp[reg_dst] == 0 {
            regs.change_flags(vec![(Flags::ZR, true), (Flags::NG, false)]);
        }
        else if regs.gp[reg_dst] < 0 {
            regs.change_flags(vec![(Flags::ZR, false), (Flags::NG, true)]);
        }

        regs.change_flags(vec![(Flags::OV, false), (Flags::CA, false)]);
    }

    // Bitwise inverts reg_src and stores the result in reg_dst
    pub fn not(regs: &mut Registers) -> () {
        let reg_dst = extract_bits(regs.ir, REG_ADDR_SIZE, REG_POS0_ROFFSET);
        let reg_src = extract_bits(regs.ir, REG_ADDR_SIZE, REG_POS1_ROFFSET);

        regs.gp[reg_dst] = !regs.gp[reg_src];
        if regs.gp[reg_dst] == 0 {
            regs.change_flags(vec![(Flags::ZR, true), (Flags::NG, false)]);
        }
        else if regs.gp[reg_dst] < 0 {
            regs.change_flags(vec![(Flags::ZR, false), (Flags::NG, true)]);
        }

        regs.change_flags(vec![(Flags::OV, false), (Flags::CA, false)]);
    }

    // Branch to address in label
    pub fn jmp(regs: &mut Registers) -> () {
        let label = extract_bits(regs.ir, MEM_LABEL_SIZE, MEM_LABEL_ROFFSET);
        regs.pc = label as u16;
    }

    // Branch with link
    pub fn bln(regs: &mut Registers) -> () {
        let label = extract_bits(regs.ir, MEM_LABEL_SIZE, MEM_LABEL_ROFFSET);
        // Save return value in link register
        regs.gp[LNR_PTR] = regs.pc as i16;
        // Do the actual jump
        regs.pc = label as u16;
    }

    // Return from branch with link
    pub fn ret(regs: &mut Registers) -> () {
        // Jumps to return address saved in link register
        regs.pc = regs.gp[LNR_PTR] as u16;
    }

    // Compare immediate
    pub fn cmp_im(regs: &mut Registers) -> () {
        let reg_a = extract_bits(regs.ir, REG_ADDR_SIZE, REG_POS0_ROFFSET);
        let val = extract_bits(regs.ir, B8_CONSTANT_SIZE, B8_CONSTANT_ROFFSET);
        
        match regs.gp[reg_a].checked_sub(val as i16) {
            Some(v) => {
                regs.change_flags(vec![(Flags::OV, false)]);
                match v {
                    v if v == 0 => regs.change_flags(vec![(Flags::CA, true),
                                                          (Flags::ZR, true),
                                                          (Flags::NG, false)]),
                    v if v > 0  => regs.change_flags(vec![(Flags::CA, true),
                                                          (Flags::ZR, false),
                                                          (Flags::NG, false)]),
                    _           => regs.change_flags(vec![(Flags::CA, false),
                                                          (Flags::ZR, false),
                                                          (Flags::NG, true)]),
                }
            }
            None => regs.change_flags(vec![(Flags::OV, true)])
        }
    }

    // Compare with registers
    pub fn cmp_rg(regs: &mut Registers) -> () {
        let reg_a = extract_bits(regs.ir, REG_ADDR_SIZE, REG_POS0_ROFFSET);
        let reg_b = extract_bits(regs.ir, REG_ADDR_SIZE, REG_POS1_ROFFSET);
        
        match regs.gp[reg_a].checked_sub(regs.gp[reg_b] as i16) {
            Some(v) => {
                regs.change_flags(vec![(Flags::OV, false)]);
                match v {
                    v if v == 0 => regs.change_flags(vec![(Flags::CA, true),
                                                          (Flags::ZR, true),
                                                          (Flags::NG, false)]),
                    v if v > 0  => regs.change_flags(vec![(Flags::CA, true),
                                                          (Flags::ZR, false),
                                                          (Flags::NG, false)]),
                    _           => regs.change_flags(vec![(Flags::CA, false),
                                                          (Flags::ZR, false),
                                                          (Flags::NG, true)]),
                }
            }
            None => regs.change_flags(vec![(Flags::OV, true)])
        }
    }

    // Branch if equal
    pub fn beq(regs: &mut Registers) -> () {
        branch_on_condition(regs.read_flag(Flags::ZR), regs);
    }

    // Branch if not equal
    pub fn bne(regs: &mut Registers) -> () {
        branch_on_condition(!regs.read_flag(Flags::ZR), regs);
    }

    // Branch if greater than (signed)
    pub fn bgt(regs: &mut Registers) -> () {
        branch_on_condition(regs.read_flag(Flags::NG) == 
                            regs.read_flag(Flags::OV),
                            regs);
    }

    // Branch if greater than (unsigned)
    pub fn bgtu(regs: &mut Registers) -> () {
        branch_on_condition(regs.read_flag(Flags::CA) && 
                            !regs.read_flag(Flags::ZR),
                            regs);
    }

    // Branch if less than (signed)
    pub fn blt(regs: &mut Registers) -> () {
        branch_on_condition(regs.read_flag(Flags::NG) != 
                            regs.read_flag(Flags::OV),
                            regs);
    }

    // Branch if less than (unsigned)
    pub fn bltu(regs: &mut Registers) -> () {
        branch_on_condition(!regs.read_flag(Flags::CA),
                            regs);
    }

    fn branch_on_condition(cond: bool, regs: &mut Registers) -> () {
        if cond {
            let label = extract_bits(regs.ir, MEM_LABEL_SIZE, MEM_LABEL_ROFFSET);
            regs.pc = label as u16;
        }
    }
}


// Helper function to extract the value of size amount of bits offseted
// by right_offset from the right.
// Example: size = 3, right_offset = 5 on num = 0110 1[011] 1000 0110 
//          yields -> 011 (decimal 7)
fn extract_bits(num: u16, size: usize, right_offset: usize) -> usize {
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