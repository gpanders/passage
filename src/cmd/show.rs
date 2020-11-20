use age::keys::Identity;
use passage::Error;
use passage::PasswordStore;
use std::fs;

pub fn show(store: &PasswordStore, item: &str) -> Result<(), Error> {
    let mut file = store.dir.join(item);
    file.set_file_name(String::from(item) + ".age");

    if !file.exists() {
        return Err(Error::ItemNotFound(String::from(item)));
    }

    let key_file = String::from(passage::data_dir().join("keys.txt").to_string_lossy());
    let key = match Identity::from_file(key_file) {
        Ok(mut identities) => match identities.pop() {
            Some(key) => key,
            _ => return Err(Error::NoSecretKey),
        },
        _ => return Err(Error::NoSecretKey),
    };

    let buf = fs::read(file).unwrap();
    let decrypted = passage::decrypt(buf, key)?;
    println!("{}", decrypted);

    Ok(())
}
