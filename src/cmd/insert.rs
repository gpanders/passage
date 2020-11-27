use age::cli_common::read_secret;
use passage::{Error, PasswordStore};
use secrecy::ExposeSecret;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::path::PathBuf;

pub fn insert(store: PasswordStore, item: Option<&str>) -> Result<(), Error> {
    if item.is_none() {
        eprintln!("Usage: passage insert ITEM");
        return Ok(());
    }

    let item = match item {
        Some(i) => i,
        _ => unreachable!(),
    };

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

    let input = match read_secret(
        &format!("Enter password for {}", item),
        "Password",
        Some(&format!("Retype password for {}", item)),
    ) {
        Ok(secret) => secret.expose_secret().clone(),
        Err(pinentry::Error::Cancelled) => return Ok(()),
        Err(pinentry::Error::Timeout) => return Err(Error::PassphraseTimedOut),
        Err(pinentry::Error::Encoding(e)) => {
            return Err(Error::IoError(io::Error::new(
                io::ErrorKind::InvalidData,
                e,
            )));
        }
        Err(pinentry::Error::Gpg(e)) => {
            return Err(Error::IoError(io::Error::new(
                io::ErrorKind::Other,
                format!("{}", e),
            )));
        }
        Err(pinentry::Error::Io(e)) => return Err(Error::IoError(e)),
    };

    let encrypted = passage::encrypt(&input, store.recipients)?;
    file.write_all(&encrypted[..])?;

    println!("Created new entry in the password store for {}.", item);
    Ok(())
}
