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

    if store.key.is_none() {
        return Err(Error::NoSecretKey);
    }

    let key = store.key.unwrap();
    let buf = fs::read(file).unwrap();
    let decrypted = passage::decrypt(buf, key)?;

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
