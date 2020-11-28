use clipboard::{ClipboardContext, ClipboardProvider};
use passage::Error;
use passage::PasswordStore;
use std::fs;
use std::path::PathBuf;

pub fn show(store: PasswordStore, item: &str, copy_to_clipboard: bool) -> Result<(), Error> {
    let file = store.dir.join(PathBuf::from(item.to_string() + ".age"));
    if !file.exists() {
        return Err(Error::ItemNotFound(item.into()));
    }

    let key_file = passage::data_dir().join("key.txt");
    let key = match passage::read_secret_key(key_file)? {
        Some(key) => key,
        None => return Err(Error::NoSecretKey),
    };

    let buf = fs::read(file).unwrap();
    let decrypted = passage::decrypt_with_key(buf, &key)?;

    if copy_to_clipboard {
        let first_line = match decrypted.split('\n').next() {
            Some(line) => line,
            None => "",
        };

        let mut ctx: ClipboardContext = ClipboardProvider::new()?;
        ctx.set_contents(first_line.to_string())?;
        println!("Copied password for {} to clipboard.", item);
    } else {
        print!("{}", decrypted);
    }

    Ok(())
}
