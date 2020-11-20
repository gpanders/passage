use age::keys::Identity;
use passage::Error;
use passage::PasswordStore;
use std::fs;

pub fn show(store: &PasswordStore, item: &str) -> Result<(), Error> {
    let mut file = store.dir.join(item);
    file.set_file_name(String::from(item) + ".age");

    if !file.exists() {
        return Err(Error::Other(format!(
            "Error: {} is not in the password store.",
            item
        )));
    }

    let key_file = String::from(passage::data_dir().join("keys.txt").to_string_lossy());
    let key = match Identity::from_file(key_file) {
        Ok(mut identities) => match identities.pop() {
            Some(key) => key,
            _ => {
                return Err(Error::Other(String::from(
                    "Error: No secret key available. You may need to run \"passage init\".",
                )));
            }
        },
        _ => {
            return Err(Error::Other(String::from(
                "Error: No secret key available. You may need to run \"passage init\".",
            )));
        }
    };

    let buf = fs::read(file).unwrap();
    let decrypted = passage::decrypt(buf, key)?;
    println!("{}", decrypted);

    Ok(())
}
