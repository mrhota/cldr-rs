use std::error;
use std::fmt;
use std::result;

use serde_json::error::Error as JsonError;
use bzip2::Error as BzipError;
use std::io::Error as IoError;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    /// Some error occurred during JSON deserialization
    Json(JsonError),
    /// Some fatal bzip error occurred during decompression.
    Bzip(BzipError),
    /// An IO error occurred while handling a CLDR data request
    Io(IoError),
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Json(ref error) => error.description(),
            Error::Bzip(ref error) => error.description(),
            Error::Io(ref error) => error.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::Json(ref error) => Some(error),
            Error::Bzip(ref error) => Some(error),
            Error::Io(ref error) => Some(error),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Json(ref error) => fmt::Display::fmt(error, fmt),
            Error::Bzip(ref error) => fmt::Display::fmt(error, fmt),
            Error::Io(ref error) => fmt::Display::fmt(error, fmt),
        }
    }
}

impl From<JsonError> for Error {
    fn from(error: JsonError) -> Error {
        Error::Json(error)
    }
}

impl From<BzipError> for Error {
    fn from(error: BzipError) -> Error {
        Error::Bzip(error)
    }
}

impl From<IoError> for Error {
    fn from(error: IoError) -> Error {
        Error::Io(error)
    }
}
