use crate::{error::Error, key, store::PasswordStore};
use age::x25519::{Identity, Recipient};
use secrecy::ExposeSecret;
use std::fs::{self, File};
use std::io::prelude::*;

pub fn init(
    mut store: PasswordStore,
    recipients: Option<Vec<Recipient>>,
    key_file: Option<String>,
) -> Result<(), Error> {
    if !store.dir.exists() {
        fs::create_dir_all(&store.dir)?;
    }

    let (old_key, new_key) = match key_file {
        Some(key_file) => {
            let key = key::read_secret_key(key_file)?;
            match key::read_secret_key(crate::key::secret_key_path()) {
                Ok(existing_key) => {
                    if existing_key.to_string().expose_secret() != key.to_string().expose_secret() {
                        (Some(existing_key), key)
                    } else {
                        (None, existing_key)
                    }
                }
                Err(Error::NoSecretKey) => (None, key),
                Err(e) => return Err(e),
            }
        }
        None => (
            None,
            match key::read_secret_key(key::secret_key_path()) {
                Ok(key) => key,
                Err(Error::NoSecretKey) => Identity::generate(),
                Err(e) => return Err(e),
            },
        ),
    };

    store.recipients.push(new_key.to_public());

    // Add additional recipients
    if let Some(recipients) = recipients {
        recipients
            .into_iter()
            .for_each(|r| store.recipients.push(r));
    }

    // Remove any duplicate recipients
    store.recipients.sort_unstable_by_key(|r| r.to_string());
    store.recipients.dedup_by_key(|r| r.to_string());

    if let Some(old_key) = &old_key {
        // Remove old public key from recipients
        let old_pubkey = old_key.to_public().to_string();
        store.recipients.retain(|k| k.to_string() != old_pubkey);

        // Re-encrypt store with the new public key
        store.reencrypt(&old_key)?;
    }

    key::save_secret_key(&new_key, key::secret_key_path(), true)?;

    let mut file = File::create(store.dir.join(".public-keys"))?;
    for recipient in &store.recipients {
        writeln!(file, "{}", recipient)?;
    }

    println!("Initialized store with the following recipients:\n");
    for recipient in &store.recipients {
        println!("    {}", recipient);
    }

    Ok(())
}
