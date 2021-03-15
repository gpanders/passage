use age::x25519::{Identity, Recipient};
use std::fs::{self, DirEntry, File, OpenOptions};
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::path::{Path, PathBuf};

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
                .filter_map(|result| result.ok())
                .filter(|line| !line.starts_with('#'))
                .map(|line| line.parse())
                .filter_map(|result| result.ok())
                .for_each(|recipient| recipients.push(recipient));
        }

        PasswordStore { dir, recipients }
    }

    pub fn exists(&self, name: &str) -> bool {
        self.dir
            .join(PathBuf::from(name.to_string() + ".age"))
            .exists()
    }

    pub fn items(&self) -> io::Result<Vec<DirEntry>> {
        fn scan(dir: &Path, entries: &mut Vec<DirEntry>) -> io::Result<()> {
            for entry in fs::read_dir(&dir)?
                .filter_map(|e| e.ok())
                .collect::<Vec<DirEntry>>()
                .into_iter()
            {
                let path = entry.path();
                if path.is_dir() {
                    scan(&path, entries)?;
                } else if let Some(ext) = path.extension() {
                    if ext == ".age" {
                        entries.push(entry);
                    }
                }
            }

            Ok(())
        }

        let mut entries: Vec<DirEntry> = vec![];
        scan(&self.dir, &mut entries)?;
        Ok(entries)
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
        for item in self.items()? {
            let mut cypher = vec![];
            File::open(item.path())?.read_to_end(&mut cypher)?;

            let secret = crypt::decrypt_with_key(&cypher, key)?;
            let encrypted = crypt::encrypt_with_keys(&secret, &self.recipients)?;
            File::create(item.path())?.write_all(&encrypted)?;
        }

        Ok(())
    }
}
