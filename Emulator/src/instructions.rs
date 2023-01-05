use num_derive::FromPrimitive;

// Opcode numerical translations
#[derive(FromPrimitive)]
pub enum Opcode {
    MovIm,  // MOV immediate (MOV {reg_dst} {constant})
    MovRg,  // MOV with registers (MOV {reg_dst} {reg_src})
    LoadIm, // LOAD immediate (LOAD {label})
    LoadRg, // LOAD with registers (LOAD {reg_dst} {reg_adr} {offset})
    StrIm,  // STR immediate (STR {label})
    StrRg,  // STR with registers (STR {reg_src} {reg_adr} {offset})
}

// Module to execute instructions
pub mod execute {
    use super::extract_bits;
    use super::super::registers::Registers;

    pub fn mov_im(ir_reg: u16, regs: &mut Registers) -> () {
        // Get the Destination Register
        let reg_dst = extract_bits(ir_reg, 3, 5);
        // Get Immediate Constant value
        let constant = extract_bits(ir_reg, 8, 8);

        match reg_dst {
            0 => regs.r0 = constant,
            1 => regs.r1 = constant,
            2 => regs.r2 = constant,
            3 => regs.r3 = constant,
            4 => regs.r4 = constant,
            5 => regs.r5 = constant,
            6 => regs.r6 = constant,
            7 => regs.r7 = constant,
            _ => panic!("Only destination registers r0 to r7 are valid"),
        }
    }


}

/* Helper function to extract the value of size amount of bits offseted
   by right_offset from the right.
   Example: size = 3, right_offset = 5 on
            num = 0110 1[011] 1000 0110 
            yields -> 011
*/
fn extract_bits(num: u16, size: u8, right_offset: u8) -> u16 {
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
    (num&mask) >> left_offset
}