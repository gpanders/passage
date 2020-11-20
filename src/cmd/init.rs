use age::keys::SecretKey;
use passage::{Error, PasswordStore};
use secrecy::ExposeSecret;
use std::fs;
use std::io::prelude::*;

fn save_secret_key(key: &SecretKey) {
    let key_dir = dirs::data_dir().unwrap().join("passage").join("keys");
    if !key_dir.exists() {
        fs::create_dir_all(&key_dir).unwrap();
    }

    fs::write(
        key_dir.join(format!("{}.txt", key.to_public())),
        key.to_string().expose_secret(),
    )
    .unwrap();
}

pub fn init(store: &PasswordStore) -> Result<(), Error> {
    if !store.dir.exists() {
        fs::create_dir_all(&store.dir).unwrap();
    }

    let key = age::SecretKey::generate();
    save_secret_key(&key);

    let mut public_keys = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(store.dir.join(".public-keys"))?;

    let pubkey = key.to_public();
    writeln!(&mut public_keys, "{}", pubkey)?;

    println!("Initialized store with new key:\n");
    println!("    {}\n", pubkey);

    Ok(())
}
