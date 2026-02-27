use edimancy::StreamParser;
use std::{fs::File, io::{BufRead, BufReader, Read}};

fn main() {

    let file_path = "tests/samples/sample837.txt";
    let file = File::open(file_path).unwrap();
    let buffered_reader = BufReader::new(file);
    let mut parser = StreamParser::new(buffered_reader, 1024, '~');

    while let Some(segment) = parser.next() {
        println!("{:?}", String::from_utf8(segment).unwrap());
    }
}