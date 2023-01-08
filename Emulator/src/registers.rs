// Constants
pub const REG_TOTAL_NUM: usize = 8; // Total num of General Purpose Registers
pub const MBR_PTR: usize = 7;       // Address of General Purpose Register MBR
pub const SP_PTR: usize = 5;        // Address of General Purpose Register SP

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
    OV(bool),
    CA(bool),
    ZR(bool),
    NG(bool)
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
    pub fn change_flags (&mut self, flags: Vec<Flags>) {
        for f in flags {

            match f {
                Flags::NG(false) => self.flags |= 1 << (Flags::NG as u8),
                Flags::NG(true)  => self.flags &= !(1 << (Flags::NG as u8)),
                Flags::ZR(false) => self.flags |= 1 << (Flags::ZR as u8),
                Flags::ZR(true)  => self.flags &= !(1 << (Flags::ZR as u8)),
                Flags::CA(false) => self.flags |= 1 << (Flags::CA as u8),
                Flags::CA(true)  => self.flags &= !(1 << (Flags::CA as u8)),
                Flags::OV(false) => self.flags |= 1 << (Flags::OV as u8),
                Flags::OV(true)  => self.flags &= !(1 << (Flags::OV as u8)),
            }
        }
    }
}