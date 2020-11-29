use passage::{Error, PasswordStore};
use std::fs;
use std::io;
use std::io::prelude::*;
use std::path::PathBuf;

pub fn insert(store: PasswordStore, item: &str) -> Result<(), Error> {
    let path = store.dir.join(PathBuf::from(item.to_string() + ".age"));
    fs::create_dir_all(&path.parent().unwrap())?;

    let mut file = match fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&path)
    {
        Ok(f) => f,
        Err(e) => match e.kind() {
            io::ErrorKind::AlreadyExists => return Err(Error::ItemAlreadyExists(item.into())),
            _ => return Err(e.into()),
        },
    };

    let password = passage::read_secret(
        &format!("Enter password for {}", item),
        Some(&format!("Retype password for {}", item)),
    )?;

    let encrypted = passage::encrypt_with_keys(&password, store.recipients)?;
    file.write_all(&encrypted)?;

    println!("Created new entry in the password store for {}.", item);
    Ok(())
}
