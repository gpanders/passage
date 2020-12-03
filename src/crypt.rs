use age::x25519::{Identity, Recipient};
use secrecy::Secret;
use std::io::prelude::*;
use std::iter;

use crate::error::Error;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encrypt_and_decrypt_with_keys() -> Result<(), Error> {
        let plaintext = "Hello world!";
        let key = Identity::generate();
        let pubkey = key.to_public();

        let encrypted = encrypt_with_keys(plaintext, &[pubkey])?;
        let decrypted = decrypt_with_key(&encrypted, &key)?;

        assert_eq!(decrypted, plaintext);

        Ok(())
    }

    #[test]
    fn encrypt_and_decrypt_with_passphrase() -> Result<(), Error> {
        let plaintext = "Testing encrypt_and_decrypt_with_passphrase";
        let passphrase = "correct horse battery staple";

        let encrypted = encrypt_with_passphrase(plaintext, passphrase)?;
        let decrypted = decrypt_with_passphrase(&encrypted, Some(passphrase))?;

        assert_eq!(decrypted, plaintext);

        Ok(())
    }
}

pub fn read_secret(prompt: &str, confirm: Option<&str>) -> Result<String, Error> {
    let input = rpassword::prompt_password_stdout(&format!("{}: ", prompt))?;

    match confirm {
        Some(prompt) => {
            if rpassword::prompt_password_stdout(&format!("{}: ", prompt))? != input {
                Err(Error::PasswordsDoNotMatch)
            } else {
                Ok(input)
            }
        }
        None => Ok(input),
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

pub fn decrypt_with_passphrase(cypher: &[u8], passphrase: Option<&str>) -> Result<String, Error> {
    let decryptor = match age::Decryptor::new(cypher) {
        Ok(d) => match d {
            age::Decryptor::Passphrase(decryptor) => decryptor,
            _ => return Err(age::DecryptError::DecryptionFailed.into()),
        },
        _ => return Err(Error::KeyNotEncrypted),
    };

    let passphrase = match passphrase {
        Some(s) => s.to_owned(),
        None => read_secret("Passphrase", None)?,
    };

    let mut decrypted = vec![];
    let mut reader = decryptor.decrypt(&Secret::new(passphrase), None)?;
    reader.read_to_end(&mut decrypted)?;

    match String::from_utf8(decrypted) {
        Ok(e) => Ok(e),
        Err(_) => Err(age::DecryptError::DecryptionFailed.into()),
    }
}

pub fn encrypt_with_keys(plaintext: &str, recipients: &[Recipient]) -> Result<Vec<u8>, Error> {
    let encryptor = age::Encryptor::with_recipients(
        recipients
            .into_iter()
            .map(|r| Box::new(r.to_owned()) as Box<dyn age::Recipient>)
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
