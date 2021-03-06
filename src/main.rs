//! Simulate files

use std::fmt;

trait Read {
    fn read(self: &Self, save_to: &mut Vec<u8>) -> Result<usize, String>;
}

/// Represents the states in which a file can reside.
#[derive(PartialEq)]
pub enum FileState {
    Closed,
    Open,
}

impl fmt::Display for FileState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FileState::Open => write!(f, "OPEN"),
            FileState::Closed => write!(f, "CLOSED"),
        }
    }
}

/// Represents a file on the file system.
pub struct File {
    pub name: String,
    data: Vec<u8>,
    pub state: FileState,
}

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{} ({})>", self.name, self.state)
    }
}

impl File {
    /// New files need a name and are assumed to be empty and closed.
    pub fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
            state: FileState::Closed,
        }
    }

    /// New files can also be created with data.
    pub fn new_with_data(name: &str, data: &Vec<u8>) -> File {
        let mut f = File::new(name);
        f.data = data.clone();
        f
    }
}

impl Read for File {
    fn read(self: &File, save_to: &mut Vec<u8>) -> Result<usize, String> {
        if self.state != FileState::Open {
            return Err(String::from("File must be open to read from it"));
        }

        let mut tmp = self.data.clone();
        let read_size = tmp.len();
        save_to.reserve(read_size);
        save_to.append(&mut tmp);

        Ok(read_size)
    }
}

fn open(mut f: File) -> Result<File, String> {
    f.state = FileState::Open;
    Ok(f)
}

fn close(mut f: File) -> Result<File, String> {
    f.state = FileState::Closed;
    Ok(f)
}

fn main() {
    let file_data = vec![114, 117, 115, 116, 33];
    let f = File::new_with_data("my-file.md", &file_data);
    println!("{}", f);

    let mut buffer: Vec<u8> = vec![];

    if f.read(&mut buffer).is_err() {
        println!("Error checking works!");
    }

    let f = open(f).unwrap();
    let f_size = f.read(&mut buffer).unwrap();
    println!("file {} has a size of {} bytes", &f.name, f_size);
    let f = close(f).unwrap();

    let txt = String::from_utf8_lossy(&buffer);
    println!("file {} has text: {}", &f.name, txt);
}
