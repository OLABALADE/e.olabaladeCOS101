use std::{fs::OpenOptions, io::Write};

fn main() {
    let mut file = OpenOptions::new().append(true).open("data.txt")
        .expect("cannot open file");

    file.write_all("\nHello Class".as_bytes()).expect("Write failed");
    file.write_all("\nThis is the appendage to the document."
        .as_bytes()).expect("write failed");
    println!("file append success");



}

