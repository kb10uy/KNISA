use std::error::Error;
use std::fmt::{Display, Error as FmtError, Formatter};

/// Represents an error in decoding code.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecodeError {
    /// Instruction length is too long.
    InvalidLength,

    /// Unknown instruction code detected.
    UnknownInstruction(u8),

    /// Specified register was out of range.
    OutOfRegisterRange(u8),
}

impl Display for DecodeError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        use DecodeError::*;
        match self {
            InvalidLength => write!(f, "Invalid instruction length"),
            UnknownInstruction(code) => write!(f, "Unknown instruction code: {:x}", code),
            OutOfRegisterRange(index) => write!(f, "Out of register range: {:x}", index),
        }
    }
}

impl Error for DecodeError {}
