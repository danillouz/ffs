#![allow(unused_variables)]

type File = String;

fn open(f: &mut File) -> bool {
    true
}

fn close(f: &mut File) -> bool {
    true
}

#[allow(dead_code)]
fn read(f: &mut File, save_to: Vec<u8>) -> ! {
    unimplemented!()
}

fn main() {
    let mut f = File::from("my-file.md");
    open(&mut f);
    // read(&mut f, vec![]);
    close(&mut f);
}
