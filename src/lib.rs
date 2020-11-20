use age::{
    self,
    keys::{Identity, RecipientKey},
};
use std::error;
use std::fmt;
use std::io;
use std::io::prelude::*;
use std::path::PathBuf;

pub use store::PasswordStore;

mod store;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encrypt_and_decrypt() {
        let plaintext = String::from("Hello world!");
        let key = age::SecretKey::generate();
        let pubkey = key.to_public();

        let encrypted = encrypt(&plaintext, vec![pubkey]).unwrap();
        let decrypted = decrypt(encrypted, key.into()).unwrap();

        assert_eq!(decrypted, plaintext);
    }
}

#[derive(Debug)]
pub enum Error {
    AgeError(age::Error),
    IoError(io::Error),
    ItemNotFound(String),
    StoreNotInitialized,
    NoSecretKey,
    Other(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::AgeError(inner) => write!(f, "Error: {}", inner),
            Error::IoError(inner) => write!(f, "Error: {}", inner),
            Error::ItemNotFound(item) => write!(f, "Error: {} is not in the password store.", item),
            Error::StoreNotInitialized => {
                write!(f, "Error: password store is empty. Try \"passage init\".")
            }
            Error::NoSecretKey => write!(f, "Error: no secret key found. Try \"passage init\"."),
            Error::Other(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::AgeError(inner) => Some(inner),
            Error::IoError(inner) => Some(inner),
            _ => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::IoError(e)
    }
}

impl From<age::Error> for Error {
    fn from(e: age::Error) -> Self {
        Error::AgeError(e)
    }
}

impl From<String> for Error {
    fn from(e: String) -> Self {
        Error::Other(e)
    }
}

pub fn data_dir() -> PathBuf {
    dirs::data_dir().unwrap().join("passage")
}

pub fn encrypt(plaintext: &str, recipients: Vec<RecipientKey>) -> Result<Vec<u8>, Error> {
    let encryptor = age::Encryptor::with_recipients(recipients);

    let mut encrypted = vec![];
    let mut writer = encryptor.wrap_output(&mut encrypted, age::Format::Binary)?;
    writer.write_all(plaintext.as_bytes())?;
    writer.finish()?;

    Ok(encrypted)
}

pub fn decrypt(cypher: Vec<u8>, key: Identity) -> Result<String, Error> {
    let decryptor = {
        match age::Decryptor::new(&cypher[..]) {
            Ok(d) => match d {
                age::Decryptor::Recipients(d) => d,
                _ => return Err(Error::AgeError(age::Error::KeyDecryptionFailed)),
            },
            Err(e) => return Err(Error::from(e)),
        }
    };

    let mut decrypted = vec![];
    let mut reader = decryptor.decrypt(&[key])?;

    reader.read_to_end(&mut decrypted)?;

    match String::from_utf8(decrypted) {
        Ok(e) => Ok(e),
        Err(_) => Err(Error::AgeError(age::Error::KeyDecryptionFailed)),
    }
}
