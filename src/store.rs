use age::x25519::{Identity, Recipient};
use std::fs::{self, DirEntry, File, OpenOptions};
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::path::PathBuf;

use crate::{crypt, error::Error, key};

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

        let encrypted = crypt::encrypt_with_keys(&secret, &self.recipients)?;
        file.write_all(&encrypted)?;

        Ok(())
    }

    pub fn get(&self, name: &str) -> Result<String, Error> {
        let path = self.dir.join(PathBuf::from(name.to_string() + ".age"));
        if !path.exists() {
            return Err(Error::ItemNotFound(name.into()));
        }

        let key = key::read_secret_key(key::secret_key_path())?;
        let decrypted = crypt::decrypt_with_key(&fs::read(path)?, &key)?;

        Ok(decrypted)
    }

    pub fn update(&self, name: &str, secret: &str) -> Result<(), Error> {
        let path = self.dir.join(PathBuf::from(name.to_string() + ".age"));
        if !path.exists() {
            return Err(Error::ItemNotFound(name.into()));
        }

        let encrypted = crypt::encrypt_with_keys(&secret, &self.recipients)?;
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

    pub fn reencrypt(&self, key: &Identity) -> Result<(), Error> {
        let items: Vec<DirEntry> = fs::read_dir(&self.dir)?
            .filter_map(|e| e.ok())
            .filter(|e| {
                e.file_name()
                    .to_str()
                    .map_or(false, |s| s.ends_with(".age"))
            })
            .collect();

        for item in items {
            let mut cypher = vec![];
            File::open(item.path())?.read_to_end(&mut cypher)?;

            let secret = crypt::decrypt_with_key(&cypher, key)?;
            let encrypted = crypt::encrypt_with_keys(&secret, &self.recipients)?;
            File::create(item.path())?.write_all(&encrypted)?;
        }

        Ok(())
    }
}
