use colored::Colorize;

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
                (Flags::NG, true)  => self.flags |= 1 << (Flags::NG as u8),
                (Flags::NG, false) => self.flags &= !(1 << (Flags::NG as u8)),
                (Flags::ZR, true)  => self.flags |= 1 << (Flags::ZR as u8),
                (Flags::ZR, false) => self.flags &= !(1 << (Flags::ZR as u8)),
                (Flags::CA, true)  => self.flags |= 1 << (Flags::CA as u8),
                (Flags::CA, false) => self.flags &= !(1 << (Flags::CA as u8)),
                (Flags::OV, true)  => self.flags |= 1 << (Flags::OV as u8),
                (Flags::OV, false) => self.flags &= !(1 << (Flags::OV as u8)),
            }
        }
    }

    pub fn read_flag(&self, flag: Flags) -> bool {
        // Set all bits except wanted bit to 0 and then shift left until
        // wanted bit is least significant bit.
        // Finally compare to 1 to either return true (if 1) or false (if 0)
        match flag {
            Flags::NG => 
                (self.flags & (1<<(Flags::NG as u8))) >> (Flags::NG as u8) == 1,
            Flags::ZR => 
                (self.flags & (1<<(Flags::ZR as u8))) >> (Flags::ZR as u8) == 1,
            Flags::CA =>
                (self.flags & (1<<(Flags::CA as u8))) >> (Flags::CA as u8) == 1,
            Flags::OV =>
                (self.flags & (1<<(Flags::OV as u8))) >> (Flags::OV as u8) == 1,
        }
    }

    pub fn print(&self) -> () {
        println!("{}", "Register Values:".bold());
        println!("REG  SIGNED UNSIGNED HEX   |  REG  SIGNED UNSIGNED HEX");
        println!("r0:  {r0:<7}{ru0:<8} {r0:0<4X}  |  fp:  {r4:<7}{ru4:<9}{r4:0<4X}", 
                  r0=self.gp[0], ru0=(self.gp[0] as u16), r4=self.gp[4], ru4=(self.gp[4] as u16));
        println!("r1:  {r1:<7}{ru1:<8} {r1:0<4X}  |  sp:  {r5:<7}{ru5:<9}{r5:0<4X}", 
                  r1=self.gp[1], ru1=(self.gp[1] as u16), r5=self.gp[5], ru5=(self.gp[5] as u16));
        println!("r2:  {r2:<7}{ru2:<8} {r2:0<4X}  |  lr:  {r6:<7}{ru6:<9}{r6:0<4X}", 
                  r2=self.gp[2], ru2=(self.gp[2] as u16), r6=self.gp[6], ru6=(self.gp[6] as u16));
        println!("r3:  {r3:<7}{ru3:<8} {r3:0<4X}  |  mbr: {r7:<7}{ru7:<9}{r7:0<4X}", 
                  r3=self.gp[3], ru3=(self.gp[3] as u16), r7=self.gp[7], ru7=(self.gp[7] as u16));
    }
}