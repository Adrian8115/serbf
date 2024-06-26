use std::fmt::Display;
use std::io::Error as IOError;
use std::sync::Arc;
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum SerbfError {
    #[error("IOError: {0}")]
    IOError(#[from] Arc<IOError>),

    #[error("{0}")]
    Message(String),

    #[error("Got an invalid enum id")]
    InvalidEnumID
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