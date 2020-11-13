use age::keys::SecretKey;
use secrecy::ExposeSecret;
use std::fs;
use std::io::prelude::*;
use std::path::Path;

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

pub fn init(store: &Path) {
    if !store.exists() {
        fs::create_dir_all(&store).unwrap();
    }

    let key = age::SecretKey::generate();
    save_secret_key(&key);

    let mut public_keys = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(store.join(".public-keys"))
        .unwrap();

    let pubkey = key.to_public();
    writeln!(&mut public_keys, "{}", pubkey).unwrap();

    println!("Initialized store with new key:\n");
    println!("    {}\n", pubkey);
}
