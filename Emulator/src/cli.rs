use colored::Colorize;
use crate::instructions::Opcode;
use num_traits::FromPrimitive;

pub struct CLI {
    pub debug: bool,
    pub file_path: Option<String>
}

impl CLI {
    pub fn new(args: Vec<String>) -> CLI {
        let mut a = CLI{debug: false, file_path: None};
        match args.len() {
            2                         => a.file_path = Some(args[1].clone()),
            3 if &args[1] == "-DEBUG" => {
                a.debug = true; a.file_path = Some(args[2].clone())
            }
            _                         => {
                println!("{} Must pass in one file as argument, and optionally pass \
                        flags", "Error".red())
            }
        }
        return a;
    }

    pub fn print_debug_welcome() {
        println!("{}", "Debug mode".green());
        println!("Press Enter key to step through code and see \
                    register values change\n");
    }

    pub fn clear_screen() {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    }

    pub fn print_curr_instruction(opcode: u8) -> () {
        let s = match FromPrimitive::from_u8(opcode) {
            Some(Opcode::MovIm)  => "MOV immediate",
            Some(Opcode::MovRg)  => "MOV with registers",
            Some(Opcode::Load)   => "LDA",
            Some(Opcode::LoadRg) => "LDR",
            Some(Opcode::Store)  => "STRA",
            Some(Opcode::StrRg)  => "STRR",
            Some(Opcode::Push)   => "PUSH",
            Some(Opcode::Pop)    => "POP",
            Some(Opcode::AddIm)  => "ADD Immediate",
            Some(Opcode::AddRg)  => "ADD with registers",
            Some(Opcode::SubIm)  => "SUB Immediate",
            Some(Opcode::SubRg)  => "SUB with registers",
            Some(Opcode::ShftL)  => "SHL",
            Some(Opcode::ShftR)  => "SHR",
            Some(Opcode::And)    => "AND",
            Some(Opcode::Or)     => "OR",
            Some(Opcode::Not)    => "NOT",
            Some(Opcode::Jmp)    => "JMP",
            Some(Opcode::Bln)    => "BLN",
            Some(Opcode::Ret)    => "RET",
            Some(Opcode::CmpIm)  => "CMP Immediate",
            Some(Opcode::CmpRg)  => "CMP with registers",
            Some(Opcode::Beq)    => "BEQ",
            Some(Opcode::Bne)    => "BNE",
            Some(Opcode::Bgt)    => "BGT",
            Some(Opcode::Bgtu)   => "BGTU",
            Some(Opcode::Blt)    => "BLT",
            Some(Opcode::Bltu)   => "BLTU",
            Some(Opcode::Halt)   => "HALT",
            _ => "Unrecognized Opcode"
        };
        println!("Current instruction: {}\n", s.bold());
    }
}