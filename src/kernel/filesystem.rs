use alloc::string::String;
use alloc::vec::Vec;

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
}
