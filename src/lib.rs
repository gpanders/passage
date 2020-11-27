use age::Identity;
use secrecy::ExposeSecret;
use std::io;
use std::io::prelude::*;
use std::iter;
use std::path::PathBuf;

pub use crate::error::Error;
pub use crate::store::PasswordStore;

mod error;
mod store;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encrypt_and_decrypt() {
        let plaintext = "Hello world!";
        let key = age::x25519::Identity::generate();
        let pubkey = key.to_public();

        let encrypted = encrypt(plaintext, vec![Box::new(pubkey)]).unwrap();
        let decrypted = decrypt(encrypted, Box::new(key)).unwrap();

        assert_eq!(decrypted, plaintext);
    }
}

pub fn data_dir() -> PathBuf {
    dirs::data_dir().unwrap().join("passage")
}

pub fn read_password(item: &str) -> Result<Option<String>, Error> {
    match age::cli_common::read_secret(
        &format!("Enter password for {}", item),
        "Password",
        Some(&format!("Retype password for {}", item)),
    ) {
        Ok(secret) => Ok(Some(secret.expose_secret().clone())),
        Err(pinentry::Error::Cancelled) => Ok(None),
        Err(pinentry::Error::Timeout) => Err(Error::PassphraseTimedOut),
        Err(pinentry::Error::Encoding(e)) => Err(Error::IoError(io::Error::new(
            io::ErrorKind::InvalidData,
            e,
        ))),
        Err(pinentry::Error::Gpg(e)) => Err(Error::IoError(io::Error::new(
            io::ErrorKind::Other,
            format!("{}", e),
        ))),
        Err(pinentry::Error::Io(e)) => Err(Error::IoError(e)),
    }
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
