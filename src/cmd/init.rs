use age::x25519::{Identity, Recipient};
use secrecy::ExposeSecret;
use std::fs::{self, File};
use std::io::prelude::*;

use crate::error::Error;
use crate::key;
use crate::store::PasswordStore;

pub fn init(
    mut store: PasswordStore,
    recipients: Option<Vec<Recipient>>,
    key_file: Option<String>,
) -> Result<(), Error> {
    if !store.dir.exists() {
        fs::create_dir_all(&store.dir)?;
    }

    let (existing_key, new_key) = match key_file {
        Some(key_file) => {
            let new_key = key::read_secret_key(key_file)?;
            match key::read_secret_key(crate::key::secret_key_path()) {
                Ok(existing_key) => (Some(existing_key), Some(new_key)),
                Err(Error::NoSecretKey) => (None, Some(new_key)),
                Err(e) => return Err(e),
            }
        }
        None => match key::read_secret_key(key::secret_key_path()) {
            Ok(existing_key) => (Some(existing_key), None),
            Err(Error::NoSecretKey) => (None, Some(Identity::generate())),
            Err(e) => return Err(e),
        },
    };

    if let Some(new_key) = &new_key {
        store.recipients.push(new_key.to_public());
    }

    // Add additional recipients
    if let Some(recipients) = recipients {
        recipients
            .into_iter()
            .for_each(|r| store.recipients.push(r));
    }

    // Remove any duplicate recipients
    store.recipients.sort_unstable_by_key(|r| r.to_string());
    store.recipients.dedup_by_key(|r| r.to_string());

    if let (Some(existing_key), Some(new_key)) = (&existing_key, &new_key) {
        if existing_key.to_string().expose_secret() != new_key.to_string().expose_secret() {
            // Remove old public key from recipients
            let pubkey = existing_key.to_public().to_string();
            store.recipients.retain(|k| k.to_string() != pubkey);
        }
    };

    if let Some(existing_key) = &existing_key {
        store.reencrypt(&existing_key)?;
    }

    if let Some(new_key) = &new_key {
        key::save_secret_key(new_key, key::secret_key_path(), true)?;
    }

    let mut file = File::create(store.dir.join(".public-keys"))?;
    for recipient in &store.recipients {
        writeln!(file, "{}", recipient)?;
    }

    eprintln!("Initialized store with the following recipients:\n");
    for recipient in &store.recipients {
        eprintln!("    {}", recipient);
    }

    Ok(())
}
