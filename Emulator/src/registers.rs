// Constants
pub const REG_TOTAL_NUM: usize = 8; // Total num of General Purpose Registers
pub const MBR_PTR: usize = 7;   // Address of General Purpose Register MBR
pub const LNR_PTR: usize = 6;   // Address of General Purpose Register Link Register
pub const SP_PTR: usize = 5;    // Address of General Purpose Register Stack Pointer

// Register type, gerneral purpose and special purpose
pub struct Registers {
    pub gp: [i16; REG_TOTAL_NUM],
    pub pc:  u16,
    pub acc: i16,
    pub ir:  u16,
    pub mar: u16,
    pub flags: u8
} 

pub enum Flags {
    OV,
    CA,
    ZR,
    NG
}

impl Registers {
    pub fn new() -> Self {
        Self {
            gp: [0; REG_TOTAL_NUM],
            pc: 0, acc: 0,
            ir: 0, mar: 0, flags: 0,
        }
    }

    //   7..4     3    2    1    0
    // | unused | NG | ZR | CA | OV |
    pub fn change_flags(&mut self, flags: Vec<(Flags, bool)>) -> () {
        for f in flags {

            match f {
                (Flags::NG, false) => self.flags |= 1 << (Flags::NG as u8),
                (Flags::NG, true)  => self.flags &= !(1 << (Flags::NG as u8)),
                (Flags::ZR, false) => self.flags |= 1 << (Flags::ZR as u8),
                (Flags::ZR, true)  => self.flags &= !(1 << (Flags::ZR as u8)),
                (Flags::CA, false) => self.flags |= 1 << (Flags::CA as u8),
                (Flags::CA, true)  => self.flags &= !(1 << (Flags::CA as u8)),
                (Flags::OV, false) => self.flags |= 1 << (Flags::OV as u8),
                (Flags::OV, true)  => self.flags &= !(1 << (Flags::OV as u8)),
            }
        }
    }

    pub fn read_flag(&self, flag: Flags) -> bool {
        // Set all bits except wanted bit to 0 and then shift left until
        // wanted bit is least significant bit.
        // Finally compare to 1 to either return true (if 1) or false (if 0)
        match flag {
            Flags::NG => 
                (self.flags & !(1<<(Flags::NG as u8))) >> (Flags::NG as u8) == 1,
            Flags::ZR => 
                (self.flags & !(1<<(Flags::ZR as u8))) >> (Flags::ZR as u8) == 1,
            Flags::CA =>
                (self.flags & !(1<<(Flags::CA as u8))) >> (Flags::CA as u8) == 1,
            Flags::OV =>
                (self.flags & !(1<<(Flags::OV as u8))) >> (Flags::OV as u8) == 1,
        }
    }
}