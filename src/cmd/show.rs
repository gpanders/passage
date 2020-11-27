use passage::Error;
use passage::PasswordStore;
use std::fs;
use std::path::PathBuf;

pub fn show(store: PasswordStore, item: &str) -> Result<(), Error> {
    let file = store.dir.join(PathBuf::from(item.to_string() + ".age"));
    if !file.exists() {
        return Err(Error::ItemNotFound(item.into()));
    }

    if store.key.is_none() {
        return Err(Error::NoSecretKey);
    }

    let key = store.key.unwrap();
    let buf = fs::read(file).unwrap();
    let decrypted = passage::decrypt(buf, key)?;
    println!("{}", decrypted);

    Ok(())
}
