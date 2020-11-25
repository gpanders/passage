use age::Identity;
use std::error;
use std::fmt;
use std::io;
use std::io::prelude::*;
use std::iter;
use std::path::PathBuf;

pub use store::PasswordStore;

mod store;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encrypt_and_decrypt() {
        let plaintext = String::from("Hello world!");
        let key = age::x25519::Identity::generate();
        let pubkey = key.to_public();

        let encrypted = encrypt(&plaintext, vec![Box::new(pubkey)]).unwrap();
        let decrypted = decrypt(encrypted, Box::new(key)).unwrap();

        assert_eq!(decrypted, plaintext);
    }
}

#[derive(Debug)]
pub enum Error {
    DecryptError(age::DecryptError),
    EncryptError(age::EncryptError),
    IoError(io::Error),
    ItemNotFound(String),
    ItemAlreadyExists(String),
    StoreNotInitialized,
    NoSecretKey,
    SecretKeyExists,
    PasswordsDoNotMatch,
    PassphraseTimedOut,
    Other(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::DecryptError(inner) => write!(f, "Error: {}", inner),
            Error::EncryptError(inner) => write!(f, "Error: {}", inner),
            Error::IoError(inner) => write!(f, "Error: {}", inner),
            Error::ItemNotFound(item) => write!(f, "Error: {} is not in the password store.", item),
            Error::ItemAlreadyExists(item) => {
                write!(f, "Error: {} already exists in the password store.", item)
            }
            Error::StoreNotInitialized => {
                write!(f, "Error: password store is empty. Try \"passage init\".")
            }
            Error::NoSecretKey => write!(f, "Error: no secret key found. Try \"passage init\"."),
            Error::SecretKeyExists => write!(f, "Error: secret key already exists."),
            Error::PasswordsDoNotMatch => write!(f, "Error: passwords do not match."),
            Error::PassphraseTimedOut => write!(f, "Error: passphrase entry timed out."),
            Error::Other(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::DecryptError(inner) => Some(inner),
            Error::EncryptError(inner) => match inner {
                age::EncryptError::Io(e) => Some(e),
            },
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

impl From<age::DecryptError> for Error {
    fn from(e: age::DecryptError) -> Self {
        Error::DecryptError(e)
    }
}

impl From<age::EncryptError> for Error {
    fn from(e: age::EncryptError) -> Self {
        Error::EncryptError(e)
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

pub fn encrypt(
    plaintext: &str,
    recipients: Vec<Box<dyn age::Recipient>>,
) -> Result<Vec<u8>, Error> {
    let encryptor = age::Encryptor::with_recipients(recipients);

    let mut encrypted = vec![];
    let mut writer = encryptor.wrap_output(&mut encrypted)?;
    writer.write_all(plaintext.as_bytes())?;
    writer.finish()?;

    Ok(encrypted)
}

pub fn decrypt(cypher: Vec<u8>, key: Box<dyn Identity>) -> Result<String, Error> {
    let decryptor = {
        match age::Decryptor::new(&cypher[..]) {
            Ok(d) => match d {
                age::Decryptor::Recipients(d) => d,
                _ => return Err(Error::DecryptError(age::DecryptError::KeyDecryptionFailed)),
            },
            Err(e) => return Err(e.into()),
        }
    };

    let mut decrypted = vec![];
    let mut reader = decryptor.decrypt(iter::once(key))?;

    reader.read_to_end(&mut decrypted)?;

    match String::from_utf8(decrypted) {
        Ok(e) => Ok(e),
        Err(_) => Err(Error::DecryptError(age::DecryptError::KeyDecryptionFailed)),
    }
}

// pub fn encrypt_store(store: &PasswordStore, key: Box<dyn Identity>) -> Result<(), Error> {
//     let items: Vec<DirEntry> = fs::read_dir(&store.dir)?
//         .filter_map(|e| e.ok())
//         .filter(|e| {
//             e.file_name()
//                 .to_str()
//                 .map_or(false, |s| s.ends_with(".age"))
//         })
//         .collect();

//     for item in items {
//         let mut file = OpenOptions::new().write(true).open(item.path())?;
//         let mut cypher = vec![];
//         file.read_to_end(&mut cypher)?;

//         let secret = decrypt(cypher, key)?;
//         let cypher = encrypt(&secret, store.recipients)?;
//         file.write_all(&cypher[..])?;
//     }

//     Ok(())
// }
