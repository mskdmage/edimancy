#[derive(Debug)]
pub enum Error {
    Generic,
    InvalidSegment(&'static str),
}