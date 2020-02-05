#![allow(unused_variables)]

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

fn open(f: &mut File) -> bool {
    true
}

fn close(f: &mut File) -> bool {
    true
}

fn read(f: &File, save_to: &mut Vec<u8>) -> usize {
    let mut tmp = f.data.clone();
    let read_size = tmp.len();
    save_to.reserve(read_size);
    save_to.append(&mut tmp);
    read_size
}

fn main() {
    let mut f = File {
        name: String::from("my-file.md"),
        data: vec![114, 117, 115, 116, 33],
    };
    println!("{:?}", f);

    open(&mut f);

    let mut buffer: Vec<u8> = vec![];
    let f_size = read(&f, &mut buffer);
    println!("file {} has a size of {} bytes", &f.name, f_size);

    let txt = String::from_utf8_lossy(&buffer);
    println!("file {} has text: {}", &f.name, txt);

    close(&mut f);
}
