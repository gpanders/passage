use clipboard::{ClipboardContext, ClipboardProvider};

use crate::error::Error;
use crate::store::PasswordStore;

pub fn show(store: PasswordStore, item: &str, copy_to_clipboard: bool) -> Result<(), Error> {
    let secret = store.get(item)?;

    if copy_to_clipboard {
        let first_line = match secret.split('\n').next() {
            Some(line) => line,
            None => "",
        };

        let mut ctx: ClipboardContext = ClipboardProvider::new()?;
        ctx.set_contents(first_line.to_string())?;
        println!("Copied password for {} to clipboard.", item);
    } else {
        print!("{}", secret);
    }

    Ok(())
}
