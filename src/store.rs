use age::{self, Identity, IdentityFile, Recipient};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;
use std::str::FromStr;

pub struct PasswordStore {
    pub dir: PathBuf,
    pub key: Option<Box<dyn Identity>>,
    pub recipients: Vec<Box<dyn Recipient>>,
}

impl PasswordStore {
    pub fn new(dir: PathBuf) -> PasswordStore {
        let mut recipients: Vec<Box<dyn Recipient>> = vec![];

        if let Ok(file) = File::open(dir.join(".public-keys")) {
            let buf = BufReader::new(file);
            buf.lines()
                .filter_map(|e| e.ok())
                .map(|e| age::x25519::Recipient::from_str(&e))
                .filter_map(|e| e.ok())
                .for_each(|e| recipients.push(Box::new(e)));
        }

        let key_file = String::from(crate::data_dir().join("key.txt").to_string_lossy());
        let key = match IdentityFile::from_file(key_file) {
            Ok(identity_file) => identity_file
                .into_identities()
                .pop()
                .map(|k| Box::new(k) as Box<dyn Identity>),
            _ => None,
        };

        PasswordStore {
            dir,
            key,
            recipients,
        }
    }
}
