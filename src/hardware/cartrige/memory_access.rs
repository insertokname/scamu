pub enum MemoryAccess {
    CpuAccess { address: u16 },
    PpuAccess { address: u16 },
}
