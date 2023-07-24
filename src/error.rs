use core::fmt::Display;
#[cfg(feature = "std")]
use std::error::Error;

#[derive(Debug, PartialEq)]
pub enum Base16384DecodeError {
    InvalidLength,
    InvalidCharacter { index: usize },
}

impl Display for Base16384DecodeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidLength => write!(f, "invalid length"),
            Self::InvalidCharacter { index } => write!(f, "invalid character at index {}", index),
        }
    }
}

#[cfg(feature = "std")]
impl Error for Base16384DecodeError {}
