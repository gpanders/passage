use crate::error::Error;
use crate::input;
use crate::store::PasswordStore;

pub fn insert(store: PasswordStore, item: Option<&str>, force: bool) -> Result<(), Error> {
    let item = match item {
        Some(s) => s.to_string(),
        None => input::read_input("Enter the name of the item you wish to add to your store.\n>")?,
    };

    if store.exists(&item) && !force {
        let ans = input::read_input(&format!(
            "{} already exists in the password store. Overwrite? [y/N]",
            item
        ))?
        .to_lowercase();
        if ans != "y" && ans != "yes" {
            return Ok(());
        }
    }

    let password = input::read_secret(
        &format!("Enter password for {}", item),
        Some(&format!("Retype password for {}", item)),
    )?;

    store.insert(&item, &password)?;
    eprintln!("Created new entry in the password store for {}.", item);
    Ok(())
}
