use crate::error::Error;
use crate::input;
use crate::store::PasswordStore;

pub fn insert(store: PasswordStore, item: Option<&str>) -> Result<(), Error> {
    let item = match item {
        Some(s) => s.to_string(),
        None => input::read_input("Enter the name of the item you wish to add to your store.\n>")?,
    };

    let password = input::read_secret(
        &format!("Enter password for {}", item),
        Some(&format!("Retype password for {}", item)),
    )?;

    store.insert(&item, &password)?;
    println!("Created new entry in the password store for {}.", item);
    Ok(())
}
