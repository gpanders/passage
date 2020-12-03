use crate::{crypt, error::Error, store::PasswordStore};

pub fn edit(store: PasswordStore, item: &str) -> Result<(), Error> {
    let password = crypt::read_secret(
        &format!("Enter new password for {}", item),
        Some(&format!("Retype new password for {}", item)),
    )?;

    store.update(item, &password)?;
    println!("Updated entry in the password store for {}.", item);
    Ok(())
}
