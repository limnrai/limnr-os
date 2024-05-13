pub trait StorageDevice {
    // read data from the storage device into a buffer
    fn read(&mut self, sector: u64, buffer: &mut [u8]) -> bool;

    // write data from a buffer to the storage device
    fn write(&mut self, sector: u64, buffer: &[u8]) -> bool;

    // get the total number of sectors on the storage device
    fn total_sectors(&self) -> u64;

    // get the sector size of the storage device
    fn sector_size(&self) -> u64;
}
