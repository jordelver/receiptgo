use std::fmt;

#[derive(Debug)]
pub enum Error {
    HttpError,
    IoError,
    Unauthorized,
    Unknown,
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::HttpError => write!(f, "HTTP error"),
            Error::Unauthorized => write!(f, "Unauthorized"),
            Error::Unknown => write!(f, "Unknown"),
            Error::IoError => write!(f, "IO error"),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(_original_error: reqwest::Error) -> Self {
        Error::HttpError
    }
}

impl From<std::io::Error> for Error {
    fn from(_original_error: std::io::Error) -> Self {
        Error::IoError
    }
}
