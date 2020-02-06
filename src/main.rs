#![allow(unused_variables)]

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

impl File {
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
        }
    }

    fn new_with_data(name: &str, data: &Vec<u8>) -> File {
        let mut f = File::new(name);
        f.data = data.clone();
        f
    }

    fn read(self: &File, save_to: &mut Vec<u8>) -> usize {
        let mut tmp = self.data.clone();
        let read_size = tmp.len();
        save_to.reserve(read_size);
        save_to.append(&mut tmp);
        read_size
    }
}

fn open(f: &mut File) -> bool {
    true
}

fn close(f: &mut File) -> bool {
    true
}

fn main() {
    let file_data = vec![114, 117, 115, 116, 33];
    let mut f = File::new_with_data("my-file.md", &file_data);
    println!("{:?}", f);

    open(&mut f);

    let mut buffer: Vec<u8> = vec![];
    let f_size = f.read(&mut buffer);
    println!("file {} has a size of {} bytes", &f.name, f_size);

    let txt = String::from_utf8_lossy(&buffer);
    println!("file {} has text: {}", &f.name, txt);

    close(&mut f);
}
