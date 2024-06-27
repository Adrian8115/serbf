use std::fmt::Display;
use std::io::Error as IOError;
use std::string::FromUtf8Error;
use std::sync::Arc;
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum SerbfError {
    #[error("IOError: {0}")]
    IOError(#[from] Arc<IOError>),

    #[error("{0}")]
    Message(String),

    #[error("Invalid enum id")]
    InvalidEnumID,

    #[error("Invalid char")]
    InvalidChar,

    #[error("UTF8 Error: {0}")]
    UTF8Error(#[from] FromUtf8Error),

    #[error("Unknown size is not supported")]
    UnknownSize,
}

impl serde::ser::Error for SerbfError {
    fn custom<T>(msg: T) -> Self
    where
        T: Display
    {
        SerbfError::Message(msg.to_string())
    }
}

impl serde::de::Error for SerbfError {
    fn custom<T>(msg: T) -> Self
    where
        T: Display
    {
        SerbfError::Message(msg.to_string())
    }
}