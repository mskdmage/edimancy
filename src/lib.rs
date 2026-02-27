pub mod parser;
pub mod components;
pub mod error;
pub mod result {
    pub type Result<T> = core::result::Result<T, crate::error::Error>;
}