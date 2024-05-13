use kernel::storage::StorageDevice;

pub struct InMemoryStorage {
    sectors: Vec<u8>,
    sector_size: u64,
}

impl InMemoryStorage {
    // create a new in-memory storage device
    pub fn new(sector_size: u64, total_sectors: u64) -> Self {
        InMemoryStorage {
            sectors: vec![0; (sector_size * total_sectors) as usize],
            sector_size,
        }
    }
}

impl StorageDevice for InMemoryStorage {
    fn read(&mut self, sector: u64, buffer: &mut [u8]) -> bool {
        let start = (sector * self.sector_size) as usize;
        let end = ((sector + 1) * self.sector_size) as usize;
        if end > self.sectors.len() || buffer.len() != self.sector_size as usize {
            return false;
        }
        buffer.copy_from_slice(&self.sectors[start..end]);
        true
    }

    fn write(&mut self, sector: u64, buffer: &[u8]) -> bool {
        let start = (sector * self.sector_size) as usize;
        let end = ((sector + 1) * self.sector_size) as usize;
        if end > self.sectors.len() || buffer.len() != self.sector_size as usize {
            return false;
        }
        self.sectors[start..end].copy_from_slice(buffer);
        true
    }

    fn total_sectors(&self) -> u64 {
        (self.sectors.len() as u64) / self.sector_size
    }

    fn sector_size(&self) -> u64 {
        self.sector_size
    }
}
