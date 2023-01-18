use std::ops::ControlFlow;

use crate::instructions::{Opcode, execute};
use crate::registers::Registers;
use num_traits::FromPrimitive;

// Fetch stage: returns the instruction in RAM specified by pc
pub fn fetch (pc: u16, ram: &Vec<u16>) -> u16 {
    ram[pc as usize]
}

// Decode stage: Obtains a 5-bit value from 16-bit register to
//               use as opcode
pub fn decode (ir_reg: u16) -> u8 {
    const OPCODE_SHIFT_OFFSET: u16 = 11;
    // 16-bit register offseted by 11 bits to retrieve
    // only the value of the 5 bits of the opcode
    (ir_reg >> OPCODE_SHIFT_OFFSET) as u8
}

// Execute Stage: Matches a given opcode to corresponding instruction
//                and executes it
pub fn execute(opcode: u8, mut regs: &mut Registers, mut mem: &mut Vec<u16>) 
-> Option<ControlFlow<()>> {
    let skip_pc_increment: Option<ControlFlow<()>> = Some(ControlFlow::Continue(()));

    match FromPrimitive::from_u8(opcode) {
        Some(Opcode::MovIm)  => {execute::mov_im(&mut regs); None}
        Some(Opcode::MovRg)  => {execute::mov_rg(&mut regs); None}
        Some(Opcode::Load)   => {execute::load(&mut regs); None}
        Some(Opcode::LoadRg) => {execute::load_rg(&mut regs, &mem); None}
        Some(Opcode::Store)  => {execute::store(&regs, &mut mem); None}
        Some(Opcode::StrRg)  => {execute::store_rg(&regs, &mut mem); None}
        Some(Opcode::Push)   => {execute::push(&mut regs, &mut mem); None}
        Some(Opcode::Pop)    => {execute::pop(&mut regs, &mem); None}
        Some(Opcode::AddIm)  => {execute::add_im(&mut regs); None}
        Some(Opcode::AddRg)  => {execute::add_rg(&mut regs); None}
        Some(Opcode::SubIm)  => {execute::sub_im(&mut regs); None}
        Some(Opcode::SubRg)  => {execute::sub_rg(&mut regs); None}
        Some(Opcode::ShftL)  => {execute::shift_l(&mut regs); None}
        Some(Opcode::ShftR)  => {execute::shift_r(&mut regs); None}
        Some(Opcode::And)    => {execute::and(&mut regs); None}
        Some(Opcode::Or)     => {execute::or(&mut regs); None}
        Some(Opcode::Not)    => {execute::not(&mut regs); None}
        Some(Opcode::Jmp)    => {execute::jmp(&mut regs); 
                                 skip_pc_increment}
        Some(Opcode::Bln)    => {execute::bln(&mut regs); 
                                 skip_pc_increment}
        Some(Opcode::Ret)    => {execute::ret(&mut regs);
                                 skip_pc_increment}
        Some(Opcode::CmpIm)  => {execute::cmp_im(&mut regs); None}
        Some(Opcode::CmpRg)  => {execute::cmp_rg(&mut regs); None}
        Some(Opcode::Beq)    => {execute::beq(&mut regs);
                                 skip_pc_increment}
        Some(Opcode::Bne)    => {execute::bne(&mut regs);
                                 skip_pc_increment}
        Some(Opcode::Bgt)    => {execute::bgt(&mut regs) ;
                                 skip_pc_increment}
        Some(Opcode::Bgtu)   => {execute::bgtu(&mut regs) ;
                                 skip_pc_increment}
        Some(Opcode::Blt)    => {execute::blt(&mut regs) ;
                                 skip_pc_increment}
        Some(Opcode::Bltu)   => {execute::bltu(&mut regs) ;
                                 skip_pc_increment}
        Some(Opcode::Halt)   => {Some(ControlFlow::Break(()))},
        _ => panic!("Unrecognized Opcode"),
    }
}