extern crate rpassword;

use passage::{Error, PasswordStore};
use std::fs;
use std::io;
use std::io::prelude::*;

pub fn insert(store: &PasswordStore, item: Option<&str>) -> Result<(), Error> {
    if item.is_none() {
        eprintln!("Usage: passage insert ITEM");
        return Ok(());
    }

    let item = match item {
        Some(i) => i,
        _ => unreachable!(),
    };

    let path = store.dir.join(String::from(item) + ".age");
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

    let prompt = format!("Enter password for {}: ", item);
    let input = rpassword::read_password_from_tty(Some(&prompt))?;

    let prompt = format!("Retype password for {}: ", item);
    if rpassword::read_password_from_tty(Some(&prompt))? != input {
        fs::remove_file(&path)?;
        return Err(Error::PasswordsDoNotMatch);
    }

    let encrypted = passage::encrypt(&input, &store.recipients)?;
    file.write_all(&encrypted[..])?;

    println!("Created new entry in the password store for {}.", item);
    Ok(())
}
