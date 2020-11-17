use std::fmt;
use std::fmt::{Debug, Formatter};
use std::io::Error;

#[derive(Debug)]
pub enum RockError {
    ProfileUncompressFailed {
        reason: String,
    },
    DecodeFieldFailed {
        reason: String,
    },
    ValidationFailed {
        reason: String,
    },
    #[allow(dead_code)]
    Unknown {
        reason: String,
    },
}

impl fmt::Display for RockError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            RockError::ValidationFailed { reason } => {
                write!(f, "Profile validation failed, reason: {}", reason)
            }
            RockError::Unknown { reason } => write!(f, "Unknown error, reason: {}", reason),
            RockError::ProfileUncompressFailed { reason } => {
                write!(f, "Failed to read compressed data. Error: {}", reason)
            }
            _ => panic!("Unknown type of error"),
        }
    }
}

impl From<RockError> for std::io::Error {
    fn from(r: RockError) -> Self {
        match r {
            RockError::ProfileUncompressFailed { reason } => {
                std::io::Error::new(std::io::ErrorKind::Other, reason)
            }
            RockError::DecodeFieldFailed { reason } => {
                std::io::Error::new(std::io::ErrorKind::Other, reason)
            }
            RockError::ValidationFailed { reason } => {
                std::io::Error::new(std::io::ErrorKind::Other, reason)
            }
            RockError::Unknown { reason } => {
                std::io::Error::new(std::io::ErrorKind::Other, reason)
            }
        }
    }
}
