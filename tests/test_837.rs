use edimancy::StreamParser;
use std::{fs::File, io::{BufRead, BufReader}};

#[test]
fn test_parse_837() {

    let file_path = "tests/samples/sample837.txt";
    let file = File::open(file_path).unwrap();
    let buffered_reader = BufReader::new(file);
    let mut parser = StreamParser::new(buffered_reader, 1024, '~');
    let mut n_segments = 0;

    while let Some(segment) = parser.next() {
        n_segments += 1;
    }
    

    assert!(n_segments == 26); // 26 Expected Segments

}