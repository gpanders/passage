use crate::{crypt, error::Error};
use age::{x25519::Identity, IdentityFile};
use secrecy::ExposeSecret;
use std::fs::{self, File, OpenOptions};
use std::io;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

#[cfg(unix)]
use std::os::unix::fs::OpenOptionsExt;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypt;
    use std::env;

    #[test]
    fn saving_and_reading_secret_key() -> Result<(), Error> {
        let plaintext = "Testing saving_and_reading_secret_key";
        let key = Identity::generate();
        let path = env::temp_dir().join("key.txt");
        let encrypted = crypt::encrypt_with_keys(plaintext, &[key.to_public()])?;

        save_secret_key(&key, &path, true)?;

        let key = read_secret_key(&path)?;
        let decrypted = crypt::decrypt_with_key(&encrypted, &key)?;

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

    let mut options = OpenOptions::new();
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

pub fn read_secret_key<P: AsRef<Path>>(path: P) -> Result<Identity, Error> {
    let path = path.as_ref();
    if !path.exists() {
        return Err(Error::NoSecretKey);
    }

    match IdentityFile::from_file(path.to_str().unwrap().to_string()) {
        Ok(identity_file) => identity_file
            .into_identities()
            .pop()
            .ok_or(Error::NoSecretKey),
        // The key file might be encrypted with a passphrase
        Err(_) => {
            let mut bytes = vec![];
            File::open(path)?.read_to_end(&mut bytes)?;

            let passphrase = crypt::read_secret("Passphrase for secret key", None)?;
            let decrypted = crypt::decrypt_with_passphrase(&bytes, Some(&passphrase))?;
            match IdentityFile::from_buffer(decrypted.as_bytes()) {
                Ok(identity_file) => identity_file
                    .into_identities()
                    .pop()
                    .ok_or(Error::NoSecretKey),
                Err(e) => Err(e.into()),
            }
        }
    }
}

pub fn encrypt_secret_key<P: AsRef<Path>>(path: P, passphrase: &str) -> Result<(), Error> {
    let key = read_secret_key(&path)?;
    let encrypted = crypt::encrypt_with_passphrase(&key.to_string().expose_secret(), &passphrase)?;

    File::create(&path)?.write_all(&encrypted)?;

    Ok(())
}

pub fn decrypt_secret_key<P: AsRef<Path>>(path: P, passphrase: Option<&str>) -> Result<(), Error> {
    let mut encrypted = vec![];
    File::open(&path)?.read_to_end(&mut encrypted)?;

    let key = IdentityFile::from_buffer(
        crypt::decrypt_with_passphrase(&encrypted, passphrase)?.as_bytes(),
    )?
    .into_identities()
    .pop()
    .ok_or_else(|| Error::NoSecretKey)?;

    save_secret_key(&key, path, true)?;

    Ok(())
}
