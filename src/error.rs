use std::error::Error as StdError;
use std::fmt;
use hex::FromHexError;
use std::option::NoneError;
use crypto::symmetriccipher::SymmetricCipherError;
use core::array::TryFromSliceError;
use std::str::Utf8Error;
use serde_json::Error as SerdeError;
use sp_core::crypto::ParseError as SpParseError;
use log;
use std::string::FromUtf8Error;
use csv::Error as CsvError;

#[derive(Debug)]
pub enum ErrorKind {
    FileNotFound,
    CallError,
    Msg(String),
    Io(::std::io::Error),
    Hex(FromHexError),
    Utf8(Utf8Error),
    Csv(CsvError),
    FromUtf8(FromUtf8Error),
    SpParse(SpParseError),
    Serde(SerdeError),
    TryFromSlice(TryFromSliceError),
    SymmetricCipher(SymmetricCipherError),
    None(NoneError),
}


/// The Error type
#[derive(Debug)]
pub struct Error {
    /// Kind of error
    pub kind: ErrorKind,
    pub source: Option<Box<dyn StdError + Send + Sync>>,
}


impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self.source {
            Some(ref err) => Some(&**err),
            _ => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            ErrorKind::Msg(ref message) => write!(f, "{:?}", message),
            ErrorKind::Io(ref e) => write!(f, "{:?}", e),
            ErrorKind::Hex(ref e) => write!(f, "{:?}", e),
            ErrorKind::Utf8(ref e) => write!(f, "{:?}", e),
            ErrorKind::FromUtf8(ref e) => write!(f, "{:?}", e),
            ErrorKind::SpParse(ref _e) => write!(f, "support network: https://github.com/paritytech/substrate/blob/master/primitives/core/src/crypto.rs"),
            ErrorKind::Csv(ref e) => write!(f, "{:?}", e),
            ErrorKind::Serde(ref e) => write!(f, "{:?}", e),
            ErrorKind::None(ref e) => write!(f, "{:?}", e),
            ErrorKind::TryFromSlice(ref _e) => write!(f, "Check the length of public key and secret key for you input"),
            ErrorKind::SymmetricCipher(ref e) => write!(f, "{:?}", e),
            ErrorKind::FileNotFound => write!(f, "The accessed file does not exist"),
            ErrorKind::CallError => write!(f, "The requested method does not exist"),
        }
    }
}


impl Error {
    pub fn msg(value: impl ToString) -> Self {
        Self { kind: ErrorKind::Msg(value.to_string()), source: None }
    }
}


impl From<&str> for Error {
    fn from(e: &str) -> Self {
        Self::msg(e)
    }
}

impl From<String> for Error {
    fn from(e: String) -> Self {
        Self::msg(e)
    }
}


impl From<FromHexError> for Error {
    fn from(e: FromHexError) -> Self {
        Self { kind: ErrorKind::Hex(e), source: None }
    }
}

impl From<Utf8Error> for Error {
    fn from(e: Utf8Error) -> Self {
        Self { kind: ErrorKind::Utf8(e), source: None }
    }
}

impl From<FromUtf8Error> for Error {
    fn from(e: FromUtf8Error) -> Self {
        Self { kind: ErrorKind::FromUtf8(e), source: None }
    }
}

impl From<SerdeError> for Error {
    fn from(e: SerdeError) -> Self {
        Self { kind: ErrorKind::Serde(e), source: None }
    }
}

// SpParseError
impl From<CsvError> for Error {
    fn from(e: CsvError) -> Self {
        Self { kind: ErrorKind::Csv(e), source: None }
    }
}

impl From<SpParseError> for Error {
    fn from(e: SpParseError) -> Self {
        Self { kind: ErrorKind::SpParse(e), source: None }
    }
}


impl From<TryFromSliceError> for Error {
    fn from(e: TryFromSliceError) -> Self {
        Self { kind: ErrorKind::TryFromSlice(e), source: None }
    }
}



impl From<SymmetricCipherError> for Error {
    fn from(e: SymmetricCipherError) -> Self {
        Self { kind: ErrorKind::SymmetricCipher(e), source: None }
    }
}

impl From<NoneError> for Error {
    fn from(e: NoneError) -> Self {
        Self { kind: ErrorKind::None(e), source: None }
    }
}

impl From<::std::io::Error> for Error {
    fn from(e: ::std::io::Error) -> Self {
        Self { kind: ErrorKind::Io(e), source: None }
    }
}


/// Prints a "backtrace" of some `Error`.
pub fn log_backtrace(e: &Error) {
    log::error!("Error: {}", e);
    // bail!("Error: {}", e);
}

pub type Result<T> = ::std::result::Result<T, Error>;