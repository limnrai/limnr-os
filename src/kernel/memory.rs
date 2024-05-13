pub struct MemoryRegion {
    start_address: usize,
    end_address: usize,
}

impl MemoryRegion {
    // create a new memory region
    pub fn new(start_address: usize, end_address: usize) -> Self {
        MemoryRegion {
            start_address,
            end_address,
        }
    }

    // check if a memory address is within this region
    pub fn contains_address(&self, address: usize) -> bool {
        address >= self.start_address && address < self.end_address
    }
}

// TODO: memory management functionality
