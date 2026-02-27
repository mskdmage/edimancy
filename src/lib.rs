pub mod error;
pub mod result {
    pub type Result<T> = core::result::Result<T, crate::error::Error>;
}

use std::io::{BufRead, BufReader};

pub struct StreamParser<T: BufRead> {
    pub reader : BufReader<T>,
    pub buffer : Vec<u8>,
    pub segment_term : u8,
}

impl<T: BufRead> StreamParser<T> {
    pub fn new(readable: T, buffer_capacity: usize, segment_term: char) -> Self {
        Self {
            reader: BufReader::new(readable),
            buffer: Vec::with_capacity(buffer_capacity as usize),
            segment_term: segment_term as u8,
        }
    }
}

impl<T: BufRead> Iterator for StreamParser<T> {
    type Item = Vec<u8>;
    
    fn next(&mut self) -> Option<Self::Item> {
        self.buffer.clear(); // Clearing for next segment.

        match self.reader.read_until(self.segment_term, &mut self.buffer) {
            Ok(0) => None, // Returns none if no bytes read.
            Ok(n) => Some(self.buffer.clone()),
            Err(e) => Some(vec![]), // Empty Vec for now.
        }                        
    }
}