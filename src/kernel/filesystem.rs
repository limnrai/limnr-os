use alloc::string::String;
use alloc::vec::Vec;
use kernel::storage::StorageDevice;

// struct to represent a file
pub struct File {
    name: String,
    contents: Vec<u8>,
}

impl File {
    // create a new file
    pub fn new(name: String) -> Self {
        File {
            name,
            contents: Vec::new(),
        }
    }

    // read the contents of the file
    pub fn read(&self) -> &[u8] {
        &self.contents
    }

    // write data to the file
    pub fn write(&mut self, data: &[u8]) {
        self.contents.clear();
        self.contents.extend_from_slice(data);
    }
}

// struct to represent a directory
pub struct Directory {
    name: String,
    files: Vec<File>,
    subdirectories: Vec<Directory>,
}

impl Directory {
    // create a new directory
    pub fn new(name: String) -> Self {
        Directory {
            name,
            files: Vec::new(),
            subdirectories: Vec::new(),
        }
    }

    // add a file to the directory
    pub fn add_file(&mut self, file: File) {
        self.files.push(file);
    }

    // subdirectory to the directory
    pub fn add_subdirectory(&mut self, directory: Directory) {
        self.subdirectories.push(directory);
    }
}

// struct to represent the file system
pub struct FileSystem {
    root: Directory,
}

impl FileSystem {
    // create a new file system
    pub fn new() -> Self {
        // Create the root directory
        let root = Directory::new(String::from("/"));

        FileSystem { root }
    }

    // get a reference to the root directory
    pub fn root(&self) -> &Directory {
        &self.root
    }

    // create a new file in the specified directory
    pub fn create_file(&mut self, path: &str, name: &str) -> Result<(), &'static str> {
        // directory where the file should be created
        let mut current_dir = &mut self.root;
        for component in path.split('/') {
            if component.is_empty() {
                continue;
            }
            match current_dir.subdirectories.iter_mut().find(|d| d.name == component) {
                Some(directory) => current_dir = directory,
                None => return Err("Directory not found"),
            }
        }

        // check file name
        if current_dir.files.iter().any(|f| f.name == name) {
            return Err("File already exists");
        }

        // create a new file and add it to the directory
        let new_file = File::new(String::from(name));
        current_dir.add_file(new_file);

        Ok(())
    }

    // delete a file from the specified directory
    pub fn delete_file(&mut self, path: &str, name: &str) -> Result<(), &'static str> {
        // find dir
        let mut current_dir = &mut self.root;
        for component in path.split('/') {
            if component.is_empty() {
                continue;
            }
            match current_dir.subdirectories.iter_mut().find(|d| d.name == component) {
                Some(directory) => current_dir = directory,
                None => return Err("Directory not found"),
            }
        }

        // find file
        if let Some(index) = current_dir.files.iter().position(|f| f.name == name) {
            current_dir.files.remove(index);
            Ok(())
        } else {
            Err("File not found")
        }
    }

    // read the contents of a file
    pub fn read_file(&self, path: &str, name: &str) -> Result<&[u8], &'static str> {
        // find dir
        let mut current_dir = &self.root;
        for component in path.split('/') {
            if component.is_empty() {
                continue;
            }
            match current_dir.subdirectories.iter().find(|d| d.name == component) {
                Some(directory) => current_dir = directory,
                None => return Err("Directory not found"),
            }
        }

        // find file
        if let Some(file) = current_dir.files.iter().find(|f| f.name == name) {
            Ok(file.read())
        } else {
            Err("File not found")
        }
    }

    // write data to a file
    pub fn write_file(&mut self, path: &str, name: &str, data: &[u8]) -> Result<(), &'static str> {
        // find dir
        let mut current_dir = &mut self.root;
        for component in path.split('/') {
            if component.is_empty() {
                continue;
            }
            match current_dir.subdirectories.iter_mut().find(|d| d.name == component) {
                Some(directory) => current_dir = directory,
                None => return Err("Directory not found"),
            }
        }

        // find file
        if let Some(file) = current_dir.files.iter_mut().find(|f| f.name == name) {
            file.write(data);
            Ok(())
        } else {
            Err("File not found")
        }
    }

    // initialize the file system
    pub fn initialize(&mut self) {
        // create the root directory
        self.root = Directory::new(String::from("/"));
    }
}

pub struct FileSystem<'a> {
    root: Directory,
    storage_device: &'a mut dyn StorageDevice,
}

impl<'a> FileSystem<'a> {
    // create a new file system
    pub fn new(storage_device: &'a mut dyn StorageDevice) -> Self {
        // root directory
        let root = Directory::new(String::from("/"));

        FileSystem {
            root,
            storage_device,
        }
    }

    // initialize the file system
    pub fn initialize(&mut self) {
        // root directory
        self.root = Directory::new(String::from("/"));
    }

}
