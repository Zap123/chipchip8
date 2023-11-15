pub struct MemoryUnit {
    pub memory: [u8; 4096]
}

impl MemoryUnit {

    pub fn new() -> MemoryUnit {
        MemoryUnit{memory: [0;4096]}
    }
}
