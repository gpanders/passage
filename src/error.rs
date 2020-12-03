use std::error;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum Error {
    ItemNotFound(String),
    ItemAlreadyExists(String),
    StoreNotInitialized,
    NoSecretKey,
    SecretKeyExists,
    KeyNotEncrypted,
    PasswordsDoNotMatch,
    Other(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::ItemNotFound(item) => write!(f, "{} is not in the password store.", item),
            Error::ItemAlreadyExists(item) => {
                write!(f, "{} already exists in the password store.", item)
            }
            Error::StoreNotInitialized => {
                write!(f, "Password store is empty. Try \"passage init\".")
            }
            Error::NoSecretKey => write!(f, "No secret key found. Try \"passage init\"."),
            Error::SecretKeyExists => {
                write!(f, "Secret key already exists. Use --force to overwrite.")
            }
            Error::KeyNotEncrypted => write!(f, "Password store is not encrypted."),
            Error::PasswordsDoNotMatch => write!(f, "Passwords do not match."),
            Error::Other(msg) => write!(f, "{}", msg),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Other(e.to_string())
    }
}

impl From<age::DecryptError> for Error {
    fn from(e: age::DecryptError) -> Self {
        Error::Other(e.to_string())
    }
}

impl From<age::EncryptError> for Error {
    fn from(e: age::EncryptError) -> Self {
        Error::Other(e.to_string())
    }
}

impl From<pinentry::Error> for Error {
    fn from(e: pinentry::Error) -> Self {
        Error::Other(e.to_string())
    }
}

impl From<String> for Error {
    fn from(e: String) -> Self {
        Error::Other(e)
    }
}

impl From<Box<dyn error::Error>> for Error {
    fn from(e: Box<dyn error::Error>) -> Self {
        Error::Other(e.to_string())
    }
}
