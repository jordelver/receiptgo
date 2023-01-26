use std::fmt;

#[derive(Debug)]
pub enum Error {
    HttpError,
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
        }
    }
}
