use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Cannot read buffer, incomplete attachment")]
    ReadError(#[from] std::io::Error),
    #[error("Cannot convert bytes to string")]
    ConversionError(#[from] std::string::FromUtf8Error),
}
