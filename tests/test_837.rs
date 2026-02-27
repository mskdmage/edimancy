use edimancy::parser::{StreamParser, parser_config_from_isa_header};
use std::io::{BufReader, Read};
use std::fs::File;

#[test]
fn test_sample837_parsing() {
    let file_path = "tests/samples/sample837.txt";
    
    let mut isa_file = File::open(file_path).expect("Failed to open sample837.txt");
    let mut isa_buf = [0u8; 106];
    isa_file.read_exact(&mut isa_buf).expect("Failed to read ISA header");
    
    let config = parser_config_from_isa_header(&isa_buf)
        .expect("Failed to parse ISA header");
    
    let file = File::open(file_path).expect("Failed to open sample837.txt");
    let reader = BufReader::new(file);
    let mut parser = StreamParser::new(reader, config.clone());
    
    let mut segment_count = 0;
    let mut element_count = 0;
    let mut found_segments = std::collections::HashSet::new();
    
    while let Some(segment_result) = parser.next() {
        match segment_result {
            Ok(segment) => {
                segment_count += 1;
                found_segments.insert(String::from_utf8_lossy(&segment.segment_id).to_string());
                
                for element_result in segment.elements(&config.segment_config, &config.element_config) {
                    let _element = element_result.expect("Failed to parse element");
                    element_count += 1;
                }
            }
            Err(e) => {
                eprintln!("Failed to parse segment: {:?}", e);
                break;
            }
        }
    }
    
    assert!(segment_count > 0, "Should parse at least one segment");
    assert!(element_count > 0, "Should parse at least one element");
    
    assert!(found_segments.contains("ISA"), "Should contain ISA segment");
    assert!(found_segments.contains("IEA"), "Should contain IEA segment");
}