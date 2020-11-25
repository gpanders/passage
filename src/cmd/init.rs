use age::x25519::Identity;
use passage::{Error, PasswordStore};
use secrecy::ExposeSecret;
use std::fs;
use std::io;
use std::io::prelude::*;

fn save_secret_key(key: &Identity) -> Result<(), Error> {
    let data_dir = passage::data_dir();
    if !data_dir.exists() {
        fs::create_dir_all(&data_dir)?;
    }

    let mut key_file = match fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(data_dir.join("key.txt"))
    {
        Ok(f) => f,
        Err(e) => match e.kind() {
            io::ErrorKind::AlreadyExists => return Err(Error::SecretKeyExists),
            _ => return Err(e.into()),
        },
    };

    key_file.write_all(key.to_string().expose_secret().as_bytes())?;

    Ok(())
}

pub fn init(store: PasswordStore) -> Result<(), Error> {
    if !store.dir.exists() {
        fs::create_dir_all(&store.dir).unwrap();
    }

    let key = age::x25519::Identity::generate();
    save_secret_key(&key)?;

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
