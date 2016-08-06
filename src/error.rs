use std::io;
use std::error;
use std::fmt;
use std::result;

use bzip2::Error as BzipError;
use serde_json::error::Error as JsonError;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    /// Some error occurred during JSON deserialization
    Json(JsonError),
    /// Some fatal bzip error occurred during decompression.
    Bzip(BzipError),
    /// An IO error occurred while handling a CLDR data request
    Io(io::Error)
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Json(ref error) => error::Error::description(error),
            Error::Bzip(ref error) => error::Error::description(error),
            Error::Io(ref error) => error::Error::description(error),
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
    fn from(error: JsonError) -> Error { Error::Json(error) }
}

impl From<BzipError> for Error {
    fn from(error: BzipError) -> Error { Error::Bzip(error) }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Error { Error::Io(error) }
}
