use age::x25519::Recipient;
use std::fs::{self, File, OpenOptions};
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::path::PathBuf;

use crate::error::Error;

pub struct PasswordStore {
    pub dir: PathBuf,
    pub recipients: Vec<Recipient>,
}

impl PasswordStore {
    pub fn new(dir: PathBuf) -> PasswordStore {
        let mut recipients: Vec<Recipient> = vec![];

        if let Ok(file) = File::open(dir.join(".public-keys")) {
            let buf = BufReader::new(file);
            buf.lines()
                .filter_map(|e| e.ok())
                .map(|e| e.parse())
                .filter_map(|e| e.ok())
                .for_each(|e| recipients.push(e));
        }

        PasswordStore { dir, recipients }
    }

    pub fn insert(&self, name: &str, secret: &str) -> Result<(), Error> {
        let path = self.dir.join(PathBuf::from(name.to_string() + ".age"));
        fs::create_dir_all(&path.parent().unwrap())?;

        let mut file = match OpenOptions::new().create_new(true).write(true).open(&path) {
            Ok(f) => f,
            Err(e) => match e.kind() {
                io::ErrorKind::AlreadyExists => return Err(Error::ItemAlreadyExists(name.into())),
                _ => return Err(e.into()),
            },
        };

        let encrypted = crate::encrypt_with_keys(&secret, &self.recipients)?;
        file.write_all(&encrypted)?;

        Ok(())
    }

    pub fn get(&self, name: &str) -> Result<String, Error> {
        let path = self.dir.join(PathBuf::from(name.to_string() + ".age"));
        if !path.exists() {
            return Err(Error::ItemNotFound(name.into()));
        }

        let key = crate::read_secret_key(crate::secret_key_path())?;
        let decrypted = crate::decrypt_with_key(&fs::read(path)?, &key)?;

        Ok(decrypted)
    }

    pub fn update(&self, name: &str, secret: &str) -> Result<(), Error> {
        let path = self.dir.join(PathBuf::from(name.to_string() + ".age"));
        if !path.exists() {
            return Err(Error::ItemNotFound(name.into()));
        }

        let encrypted = crate::encrypt_with_keys(&secret, &self.recipients)?;
        File::create(path)?.write_all(&encrypted)?;

        Ok(())
    }

    pub fn delete(&self, name: &str) -> Result<(), Error> {
        if let Err(e) = fs::remove_file(self.dir.join(name.to_string() + ".age")) {
            match e.kind() {
                io::ErrorKind::NotFound => return Err(Error::ItemNotFound(name.into())),
                _ => return Err(e.into()),
            }
        }

        Ok(())
    }
}
