use passage::{Error, PasswordStore};
use std::fs;
use std::io;
use std::io::prelude::*;

pub fn remove(store: PasswordStore, item: Option<&str>) -> Result<(), Error> {
    if item.is_none() {
        eprintln!("Usage: passage rm ITEM");
        return Ok(());
    }

    let item = match item {
        Some(i) => i,
        _ => unreachable!(),
    };

    print!("Delete {}? [y/N] ", item);
    io::stdout().flush()?;

    let mut ans = String::new();
    io::stdin().read_line(&mut ans)?;

    let ans = ans.trim_end().to_lowercase();
    if ans != "y" && ans != "yes" {
        return Ok(());
    }

    if let Err(e) = fs::remove_file(store.dir.join(String::from(item) + ".age")) {
        match e.kind() {
            io::ErrorKind::NotFound => {
                eprintln!("{} does not exist in the password store.", item);
                return Ok(());
            }
            _ => return Err(e.into()),
        }
    }

    println!("Removed item {}.", item);
    Ok(())
}
