pub struct Registers {
    pub r0: u16,
    pub r1: u16,
    pub r2: u16,
    pub r3: u16,
    pub r4: u16,
    pub r5: u16,
    pub r6: u16,
    pub r7: u16,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            r0: 0, r1: 0, r2: 0, r3: 0,
            r4: 0, r5: 0, r6: 0, r7: 0
        }
    }
}