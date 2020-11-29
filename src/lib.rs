use age::{
    x25519::{Identity, Recipient},
    IdentityFile,
};
use secrecy::{ExposeSecret, SecretString};
use std::fs::{self, File};
use std::io;
use std::io::prelude::*;
use std::iter;
use std::path::{Path, PathBuf};

#[cfg(unix)]
use std::os::unix::fs::OpenOptionsExt;

mod error;
pub use crate::error::Error;

mod store;
pub use crate::store::PasswordStore;

#[cfg(test)]
mod tests {
    use super::*;
    use secrecy::Secret;
    use std::env;

    #[test]
    fn encrypt_and_decrypt_with_keys() -> Result<(), Error> {
        let plaintext = "Hello world!";
        let key = Identity::generate();
        let pubkey = key.to_public();

        let encrypted = encrypt_with_keys(plaintext, vec![pubkey])?;
        let decrypted = decrypt_with_key(&encrypted, &key)?;

        assert_eq!(decrypted, plaintext);

        Ok(())
    }

    #[test]
    fn encrypt_and_decrypt_with_passphrase() -> Result<(), Error> {
        let plaintext = "Testing encrypt_and_decrypt_with_passphrase";
        let passphrase = Secret::new("correct horse battery staple".to_string());

        let encrypted = encrypt_with_passphrase(plaintext, &passphrase)?;
        let decrypted = decrypt_with_passphrase(&encrypted, &passphrase)?;

        assert_eq!(decrypted, plaintext);

        Ok(())
    }

    #[test]
    fn saving_and_reading_secret_key() -> Result<(), Error> {
        let plaintext = "Testing saving_and_reading_secret_key";
        let key = Identity::generate();
        let path = env::temp_dir().join("key.txt");
        let encrypted = encrypt_with_keys(plaintext, vec![key.to_public()])?;

        save_secret_key(&key, &path, true)?;

        let key = read_secret_key(&path)?;
        let decrypted = decrypt_with_key(&encrypted, &key.unwrap())?;

        assert_eq!(decrypted, plaintext);

        Ok(())
    }
}

pub fn secret_key_path() -> PathBuf {
    dirs::data_dir().unwrap().join("passage").join("key.txt")
}

pub fn save_secret_key<P: AsRef<Path>>(key: &Identity, path: P, force: bool) -> Result<(), Error> {
    let path = path.as_ref();
    if !path.exists() {
        fs::create_dir_all(path.parent().unwrap())?;
    }

    let mut options = fs::OpenOptions::new();
    options.write(true).truncate(true);

    if force {
        options.create(true);
    } else {
        options.create_new(true);
    }

    #[cfg(unix)]
    options.mode(0o600);

    let mut key_file = match options.open(&path) {
        Ok(f) => f,
        Err(e) => match e.kind() {
            io::ErrorKind::AlreadyExists => return Err(Error::SecretKeyExists),
            _ => return Err(e.into()),
        },
    };

    key_file.write_all(key.to_string().expose_secret().as_bytes())?;

    Ok(())
}

pub fn read_secret_key<P: AsRef<Path>>(path: P) -> Result<Option<Identity>, Error> {
    let path = path.as_ref();
    if !path.exists() {
        return Ok(None);
    }

    match IdentityFile::from_file(path.to_str().unwrap().to_string()) {
        Ok(identity_file) => Ok(identity_file.into_identities().pop()),
        // The key file might be encrypted with a passphrase
        Err(_) => {
            let mut bytes = vec![];
            File::open(path)?.read_to_end(&mut bytes)?;

            let passphrase =
                age::cli_common::read_secret("Passphrase for secret key", "Passphrase", None)?;
            let decrypted = decrypt_with_passphrase(&bytes, &passphrase)?;
            match IdentityFile::from_buffer(decrypted.as_bytes()) {
                Ok(identity_file) => Ok(identity_file.into_identities().pop()),
                Err(e) => Err(e.into()),
            }
        }
    }
}

pub fn encrypt_secret_key<P: AsRef<Path>>(path: P, passphrase: &SecretString) -> Result<(), Error> {
    let key = read_secret_key(&path)?.ok_or_else(|| Error::NoSecretKey)?;

    let encrypted = encrypt_with_passphrase(&key.to_string().expose_secret(), &passphrase)?;

    File::create(&path)?.write_all(&encrypted)?;

    Ok(())
}

pub fn decrypt_secret_key<P: AsRef<Path>>(path: P, passphrase: &SecretString) -> Result<(), Error> {
    let mut encrypted = vec![];
    File::open(&path)?.read_to_end(&mut encrypted)?;

    let key =
        IdentityFile::from_buffer(decrypt_with_passphrase(&encrypted, passphrase)?.as_bytes())?
            .into_identities()
            .pop()
            .ok_or_else(|| Error::NoSecretKey)?;

    save_secret_key(&key, path, true)?;

    Ok(())
}

pub fn encrypt_with_passphrase(
    plaintext: &str,
    passphrase: &SecretString,
) -> Result<Vec<u8>, Error> {
    let encryptor = age::Encryptor::with_user_passphrase(passphrase.to_owned());
    let mut encrypted = vec![];
    let mut writer = encryptor.wrap_output(&mut encrypted)?;
    writer.write_all(plaintext.as_bytes())?;
    writer.finish()?;

    Ok(encrypted)
}

pub fn decrypt_with_passphrase(cypher: &[u8], passphrase: &SecretString) -> Result<String, Error> {
    let decryptor = match age::Decryptor::new(cypher)? {
        age::Decryptor::Passphrase(decryptor) => decryptor,
        _ => return Err(age::DecryptError::DecryptionFailed.into()),
    };

    let mut decrypted = vec![];
    let mut reader = decryptor.decrypt(passphrase, None)?;
    reader.read_to_end(&mut decrypted)?;

    match String::from_utf8(decrypted) {
        Ok(e) => Ok(e),
        Err(_) => Err(age::DecryptError::DecryptionFailed.into()),
    }
}

pub fn encrypt_with_keys(plaintext: &str, recipients: Vec<Recipient>) -> Result<Vec<u8>, Error> {
    let encryptor = age::Encryptor::with_recipients(
        recipients
            .into_iter()
            .map(|r| Box::new(r) as Box<dyn age::Recipient>)
            .collect(),
    );

    let mut encrypted = vec![];
    let mut writer = encryptor.wrap_output(&mut encrypted)?;
    writer.write_all(plaintext.as_bytes())?;
    writer.finish()?;

    Ok(encrypted)
}

pub fn decrypt_with_key(cypher: &[u8], key: &Identity) -> Result<String, Error> {
    let decryptor = {
        match age::Decryptor::new(cypher) {
            Ok(d) => match d {
                age::Decryptor::Recipients(d) => d,
                _ => return Err(age::DecryptError::KeyDecryptionFailed.into()),
            },
            Err(e) => return Err(e.into()),
        }
    };

    let mut decrypted = vec![];
    let mut reader = decryptor.decrypt(iter::once(
        Box::new(key.to_owned()) as Box<dyn age::Identity>
    ))?;

    reader.read_to_end(&mut decrypted)?;

    match String::from_utf8(decrypted) {
        Ok(e) => Ok(e),
        Err(_) => Err(age::DecryptError::KeyDecryptionFailed.into()),
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
//         file.write_all(&cypher)?;
//     }

//     Ok(())
// }
