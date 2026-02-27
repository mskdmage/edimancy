use std::{fs::File, io::{BufRead, BufReader, Read}};

fn main() {

    let file_path = "tests/samples/sample837.txt";
    let file = File::open(file_path).unwrap();
    let mut buffered_reader = BufReader::new(file);
    let mut buffer = Vec::with_capacity(1024);
    buffered_reader.read_until(b'~',&mut buffer).unwrap();

    println!("{:?}", String::from_utf8(buffer).unwrap());
}