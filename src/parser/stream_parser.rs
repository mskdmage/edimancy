use std::io::{BufReader, BufRead};

use crate::result::Result;
use crate::components::{Segment, SegmentConfig,ElementConfig};

pub struct StreamParser<T: BufRead> {
    config: ParserConfig,
    reader: BufReader<T>,
    buffer: Vec<u8>,
}

impl<T: BufRead> StreamParser<T> {
    pub fn new(reader: T, config: ParserConfig) -> Self {
        Self {
            config,
            reader: BufReader::with_capacity(256 * 1024, reader),
            buffer: Vec::with_capacity(4096),
        }
    }
}

impl<T: BufRead> Iterator for StreamParser<T> {
    type Item = Result<Segment>;

    fn next(&mut self) -> Option<Self::Item> {
        
        self.buffer.clear();

        let seg_term = self.config.segment_config.terminator;

        match self.reader.read_until(seg_term, &mut self.buffer) {
            Ok(0) => None,
            Ok(_) => {
                Some(Segment::from_bytes(
                    self.buffer.as_slice(),
                    &self.config.segment_config,
                ))
            }
            Err(_) => Some(Err(crate::error::Error::InvalidSegment(
                "io error while reading segment",
            ))),
        }
    }
}

#[derive(Clone)]
pub struct ParserConfig {
    pub segment_config: SegmentConfig,
    pub element_config: ElementConfig,
}

impl ParserConfig {
    pub fn new(
        segment_terminator: char,
        element_separator: char,
        component_element_separator: char,
        repetition_separator: char,
    ) -> Self {
        Self {
            segment_config: SegmentConfig {
                terminator: segment_terminator as u8,
                element_separator: element_separator as u8,
            },
            element_config: ElementConfig {
                subelement_separator: component_element_separator as u8,
                repetition_separator: repetition_separator as u8,
            },
        }
    }

    pub fn from_isa_header(isa_bytes: &[u8]) -> Result<Self> {
        if isa_bytes.len() < 106 {
            return Err(crate::error::Error::InvalidSegment("ISA header must be at least 106 bytes"));
        }

        let element_separator = isa_bytes[3];
        let subelement_separator = isa_bytes[104];
        let segment_terminator = isa_bytes[105];

        Ok(Self {
            segment_config: SegmentConfig {
                terminator: segment_terminator,
                element_separator,
            },
            element_config: ElementConfig {
                subelement_separator,
                repetition_separator: b'^',
            },
        })
    }
}