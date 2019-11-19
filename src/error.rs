use std::io;
use std::result;

#[derive(Debug)]
pub enum Error {
    IOError(io::Error),
    LZMAError(LZMAError),
    XZError(String),
}

pub type Result<T> = result::Result<T, Error>;

impl Error {
    pub fn lzma_other<T: Into<String>>(msg: T) -> Error {
        Error::LZMAError(LZMAError::other(msg))
    }

    pub fn has_extra_bytes(&self) -> bool {
        match self {
            Error::LZMAError(LZMAError::ExtraBytes) => true,
            _ => false,
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error::IOError(e)
    }
}

impl From<LZMAError> for Error {
    fn from(e: LZMAError) -> Error {
        Error::LZMAError(e)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use Error::*;

        match self {
            IOError(e) => write!(f, "I/O error: {}", e),
            LZMAError(e)=> write!(f, "LZMA error: {}", e),
            XZError(e)=> write!(f, "XZError error: {}", e),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use Error::*;

        match self {
            IOError(e) => Some(e),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum LZMAError {
    ExtraBytes,
    Other(String),
}

impl LZMAError {
    pub fn other<T: Into<String>>(msg: T) -> LZMAError
    {
        LZMAError::Other(msg.into())
    }
}

impl std::fmt::Display for LZMAError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use LZMAError::*;

        match self {
            ExtraBytes => write!(f, "More bytes after end of stream"),
            Other(e)=> write!(f, "{}", e),
        }
    }
}

impl std::error::Error for LZMAError { }
