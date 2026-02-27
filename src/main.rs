use edimancy::{parser::{StreamParser, parser_config_from_isa_header}};
use std::{fs::File, io::{BufReader, Read}};

fn main() {

    let file_path = "tests/samples/sample837.txt";
    let file = File::open(file_path).unwrap();
    let mut buffered_reader = BufReader::new(file);
    let mut isa_buf = [0u8; 106];
    buffered_reader.read_exact(&mut isa_buf).expect("Failed to read ISA header");
    let config = parser_config_from_isa_header(&isa_buf).unwrap();

    let mut parser = StreamParser::new(buffered_reader, config);

    while let Some(segment) = parser.next() {
        println!("{:?}", String::from_utf8(segment.unwrap().body).unwrap());
    }
}