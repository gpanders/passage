use passage::Error;
use passage::PasswordStore;
use std::fs;

pub fn show(store: PasswordStore, item: &str) -> Result<(), Error> {
    let mut file = store.dir.join(item);
    file.set_file_name(String::from(item) + ".age");

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
