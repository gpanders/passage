use age::{
    x25519::{Identity, Recipient},
    IdentityFile,
};
use secrecy::{ExposeSecret, Secret};
use std::fs::{self, File};
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::iter;
use std::path::{Path, PathBuf};

#[cfg(unix)]
use std::os::unix::fs::OpenOptionsExt;

pub use crate::error::Error;
pub use crate::store::PasswordStore;

mod error;
mod store;

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn encrypt_and_decrypt_with_keys() -> Result<(), Error> {
        let plaintext = "Hello world!";
        let key = Identity::generate();
        let pubkey = key.to_public();

        let encrypted = encrypt_with_keys(plaintext, vec![pubkey])?;
        let decrypted = decrypt_with_key(encrypted, &key)?;

        assert_eq!(decrypted, plaintext);

        Ok(())
    }

    #[test]
    fn encrypt_and_decrypt_with_passphrase() -> Result<(), Error> {
        let plaintext = "Testing encrypt_and_decrypt_with_passphrase";
        let passphrase = "correct horse battery staple";

        let encrypted = encrypt_with_passphrase(plaintext, passphrase)?;
        let decrypted = decrypt_with_passphrase(encrypted, Some(passphrase))?;

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
        let decrypted = decrypt_with_key(encrypted, &key.unwrap())?;

        assert_eq!(decrypted, plaintext);

        Ok(())
    }
}

pub fn data_dir() -> PathBuf {
    dirs::data_dir().unwrap().join("passage")
}

pub fn save_secret_key<P: AsRef<Path>>(key: &Identity, path: P, force: bool) -> Result<(), Error> {
    let path = path.as_ref();
    if !path.exists() {
        fs::create_dir_all(path.parent().unwrap())?;
    }

    let mut options = fs::OpenOptions::new();
    options.write(true);

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

            let decrypted = decrypt_with_passphrase(bytes, None)?;
            match IdentityFile::from_buffer(BufReader::new(decrypted.as_bytes())) {
                Ok(identity_file) => Ok(identity_file.into_identities().pop()),
                Err(e) => Err(e.into()),
            }
        }
    }
}

pub fn read_password(item: &str) -> Result<String, Error> {
    match age::cli_common::read_secret(
        &format!("Enter password for {}", item),
        "Password",
        Some(&format!("Retype password for {}", item)),
    ) {
        Ok(secret) => Ok(secret.expose_secret().clone()),
        Err(e) => Err(e.into()),
    }
}

pub fn encrypt_with_passphrase(plaintext: &str, passphrase: &str) -> Result<Vec<u8>, Error> {
    let encryptor = age::Encryptor::with_user_passphrase(Secret::new(passphrase.to_owned()));
    let mut encrypted = vec![];
    let mut writer = encryptor.wrap_output(&mut encrypted)?;
    writer.write_all(plaintext.as_bytes())?;
    writer.finish()?;

    Ok(encrypted)
}

pub fn decrypt_with_passphrase(cypher: Vec<u8>, passphrase: Option<&str>) -> Result<String, Error> {
    let decryptor = match age::Decryptor::new(&cypher[..])? {
        age::Decryptor::Passphrase(decryptor) => decryptor,
        _ => return Err(age::DecryptError::DecryptionFailed.into()),
    };

    let passphrase = match passphrase {
        Some(p) => Secret::new(p.to_owned()),
        None => age::cli_common::read_secret("Passphrase", "Passphrase", None)?,
    };

    let mut decrypted = vec![];
    let mut reader = decryptor.decrypt(&passphrase, None)?;
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

pub fn decrypt_with_key(cypher: Vec<u8>, key: &Identity) -> Result<String, Error> {
    let decryptor = {
        match age::Decryptor::new(&cypher[..]) {
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
//         file.write_all(&cypher[..])?;
//     }

//     Ok(())
// }
