use age::keys::RecipientKey;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;
use std::str::FromStr;

pub struct PasswordStore {
    pub dir: PathBuf,
    pub recipients: Vec<RecipientKey>,
}

impl PasswordStore {
    pub fn new(dir: PathBuf) -> PasswordStore {
        let mut recipients: Vec<RecipientKey> = vec![];

        if let Ok(file) = File::open(dir.join(".public-keys")) {
            let buf = BufReader::new(file);
            buf.lines()
                .filter_map(|e| e.ok())
                .map(|e| RecipientKey::from_str(&e))
                .filter_map(|e| e.ok())
                .for_each(|e| recipients.push(e));
        }

        PasswordStore { dir, recipients }
    }
}
