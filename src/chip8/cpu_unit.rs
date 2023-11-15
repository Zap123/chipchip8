pub struct CpuUnit {
    // 15 registers + carry flag, 8bit
    pub v:[u8;16],
    // program counter (0x000, 0xFFF)
    pub pc: u16,
    // index register
    pub i: u16,
    // the stack
    pub stack: [u16; 16],
    pub sp: u16,
}

impl CpuUnit {
    pub fn new() -> CpuUnit {
        CpuUnit{v: [0; 16], pc: 0x200, i: 0, stack: [0; 16], sp:0 }
    }

}
