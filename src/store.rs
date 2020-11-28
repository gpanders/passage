use age::{self, x25519::Recipient};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;
use std::str::FromStr;

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
                .map(|e| age::x25519::Recipient::from_str(&e))
                .filter_map(|e| e.ok())
                .for_each(|e| recipients.push(e));
        }

        PasswordStore { dir, recipients }
    }
}
