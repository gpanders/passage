use age::{
    self,
    keys::{RecipientKey, SecretKey},
};
use std::fs;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

pub use store::PasswordStore;

mod store;

pub fn data_dir() -> PathBuf {
    dirs::data_dir().unwrap().join("passage")
}

pub fn encrypt(
    secret: String,
    file: &Path,
    recipients: Vec<RecipientKey>,
) -> Result<(), &'static str> {
    let encryptor = age::Encryptor::with_recipients(recipients);

    let output = match fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(file)
    {
        Ok(output) => output,
        Err(e) => return Err(&format!("Error: Couldn't create file: {}", e)),
    };

    let mut writer = match encryptor.wrap_output(output, age::Format::Binary) {
        Ok(e) => e,
        Err(e) => return Err(&format!("Error: Couldn't create an encryptor: {}", e)),
    };

    match writer.write_all(secret.as_bytes()) {
        Err(e) => return Err(&format!("Error: Failed to write encrypted file: {}", e)),
        Ok(_) => writer.finish(),
    };

    Ok(())
}

pub fn decrypt(file: &Path, key: SecretKey) -> Result<String, &'static str> {
    let buf = match fs::read(file) {
        Ok(b) => b,
        Err(_) => return Err("Couldn't read file"),
    };

    let decryptor = {
        match age::Decryptor::new(&buf[..]) {
            Ok(d) => match d {
                age::Decryptor::Recipients(d) => d,
                _ => {
                    return Err(&format!(
                        "Error: couldn't decrypt file `{}': not encrypted with a public key",
                        file.display()
                    ))
                }
            },
            Err(e) => {
                return Err(&format!(
                    "Error: couldn't decrypt file `{}': {}",
                    file.display(),
                    e
                ))
            }
        }
    };

    let mut decrypted = vec![];
    let mut reader = match decryptor.decrypt(&[key.into()]) {
        Ok(reader) => reader,
        Err(e) => {
            return Err(&format!(
                "Error: failed to decrypt file `{}': {}",
                file.display(),
                e
            ))
        }
    };

    match reader.read_to_end(&mut decrypted) {
        Ok(_) => {}
        Err(e) => {
            return Err(&format!(
                "Error: failed to read decrypted file `{}': {}",
                file.display(),
                e
            ))
        }
    };

    match String::from_utf8(decrypted) {
        Ok(e) => Ok(e),
        Err(_) => Err("Error: decrypted data couldn't be parsed as UTF-8"),
    }
}
