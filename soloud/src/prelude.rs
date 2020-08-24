use std::convert::From;
use std::{fmt, io};

#[derive(Debug)]
pub enum SoloudError {
    IoError(io::Error),
    NullError(std::ffi::NulError),
    Internal(SoloudErrorKind),
    Unknown(String),
}

/// Error kinds enum for SoloudError
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SoloudErrorKind {
    FailedToRun,
    FailedToLock,
    FailedToSetScheme,
    FailedOperation,
    ResourceNotFound,
    ImageFormatError,
    TableError,
}

impl std::error::Error for SoloudError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            SoloudError::IoError(err) => Some(err),
            SoloudError::NullError(err) => Some(err),
            _ => None,
        }
    }
}

impl fmt::Display for SoloudError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SoloudError::IoError(ref err) => err.fmt(f),
            SoloudError::NullError(ref err) => err.fmt(f),
            SoloudError::Internal(ref err) => write!(f, "An internal error occured {:?}", err),
            SoloudError::Unknown(ref err) => write!(f, "An unknown error occurred {:?}", err),
        }
    }
}

impl From<io::Error> for SoloudError {
    fn from(err: io::Error) -> SoloudError {
        SoloudError::IoError(err)
    }
}

impl From<std::ffi::NulError> for SoloudError {
    fn from(err: std::ffi::NulError) -> SoloudError {
        SoloudError::NullError(err)
    }
}