use std::io::{BufRead, BufReader};
use crate::result::Result;
use crate::components::{Segment, SegmentConfig};

pub struct StreamParser<T: BufRead> {
    pub reader : BufReader<T>,
    pub buffer : Vec<u8>,
    pub segment_config : SegmentConfig,
}

impl<T: BufRead> StreamParser<T> {
    pub fn new(readable: T, buffer_capacity: usize, segment_config: SegmentConfig) -> Self {
        Self {
            reader: BufReader::new(readable),
            buffer: Vec::with_capacity(buffer_capacity as usize),
            segment_config,
        }
    }
}

impl<T: BufRead> Iterator for StreamParser<T> {
    type Item = Result<Segment>;
    
    fn next(&mut self) -> Option<Self::Item> {
        self.buffer.clear(); // Clearing for next segment.

        match self.reader.read_until(self.segment_config.terminator, &mut self.buffer) {
            Ok(0) => None,
            Ok(_) => {
                Some(Segment::from_bytes(
                    self.buffer.as_slice(),
                    &self.segment_config,
                ))
            }
            Err(_) => Some(Err(crate::error::Error::InvalidSegment(
                "io error while reading segment",
            ))),
        }
    }
}