use kernel::filesystem::FileSystem;
use kernel::storage::InMemoryStorage;

pub fn init() {
    // create a storage device (in-memory)
    let mut storage_device = InMemoryStorage::new(512, 1024);

    // initialize the file system
    let mut file_system = FileSystem::new();
    file_system.initialize();

}
