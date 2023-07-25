//! Error types.

use core::fmt::Display;
#[cfg(feature = "std")]
use std::error::Error;

/// Errors that can occur when decoding base16384.
#[derive(Debug, PartialEq)]
pub enum Base16384DecodeError {
    /// The input data has an invalid length.
    InvalidLength,
    /// The input data has an invalid character at the given index.
    InvalidCharacter {
        /// The index of the invalid character.
        ///
        /// In UTF-8, this is the byte index.
        index: usize,
    },
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
