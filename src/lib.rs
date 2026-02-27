pub mod error;
mod result {
    pub type Result<T> = core::result::Result<T, crate::error::Error>;
}