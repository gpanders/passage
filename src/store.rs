use age::keys::RecipientKey;
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

pub struct PasswordStore {
    pub dir: PathBuf,
    pub recipients: Vec<RecipientKey>,
}

impl PasswordStore {
    pub fn new(dir: PathBuf) -> PasswordStore {
        let mut recipients: Vec<RecipientKey> = Vec::new();

        if let Ok(data) = fs::read(dir.join(".public-keys")) {
            let contents = String::from_utf8_lossy(&data);
            let public_keys = contents.split('\n');
            public_keys
                .map(|e| RecipientKey::from_str(e))
                .filter_map(|e| e.ok())
                .for_each(|e| recipients.push(e));
        };

        PasswordStore { dir, recipients }
    }
}
