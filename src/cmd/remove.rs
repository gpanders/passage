use crate::{error::Error, store::PasswordStore};
use std::io;
use std::io::prelude::*;

pub fn remove(store: PasswordStore, item: &str) -> Result<(), Error> {
    print!("Delete {}? [y/N] ", item);
    io::stdout().flush()?;

    let mut ans = String::new();
    io::stdin().read_line(&mut ans)?;

    let ans = ans.trim_end().to_lowercase();
    if ans != "y" && ans != "yes" {
        return Ok(());
    }

    store.delete(item)?;
    println!("Removed item {}.", item);
    Ok(())
}
