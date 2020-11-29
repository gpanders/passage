use passage::{Error, PasswordStore};
use std::fs;
use std::io::prelude::*;

pub fn init(store: PasswordStore) -> Result<(), Error> {
    if !store.dir.exists() {
        fs::create_dir_all(&store.dir).unwrap();
    }

    let key = age::x25519::Identity::generate();
    passage::save_secret_key(&key, passage::secret_key_path(), false)?;

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
