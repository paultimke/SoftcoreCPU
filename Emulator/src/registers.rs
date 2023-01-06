// Constants
pub const REG_TOTAL_NUM: usize = 8; // Total num of General Purpose Registers
pub const MBR_PTR: usize = 7;       // Address of General Purpose Register MBR
pub const SP_PTR: usize = 5;        // Address of General Purpose Register SP

// Register type, gerneral purpose and special purpose
pub struct Registers {
    pub gp: [u16; REG_TOTAL_NUM],
    pub pc:  u16,
    pub acc: u16,
    pub ir:  u16,
    pub mar: u16,
} 

impl Registers {
    pub fn new() -> Self {
        Self {
            gp: [0; REG_TOTAL_NUM],
            pc: 0, acc: 0,
            ir: 0, mar: 0
        }
    }
}