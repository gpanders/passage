use passage::{Error, PasswordStore};

pub fn insert(store: PasswordStore, item: &str) -> Result<(), Error> {
    let password = passage::read_secret(
        &format!("Enter password for {}", item),
        Some(&format!("Retype password for {}", item)),
    )?;

    store.insert(item, &password)?;
    println!("Created new entry in the password store for {}.", item);
    Ok(())
}
