use crate::error::Error;
use crate::input;
use crate::store::PasswordStore;

pub fn edit(store: PasswordStore, item: Option<&str>) -> Result<(), Error> {
    let item = match item {
        Some(s) => s.to_string(),
        None => {
            input::read_input("Enter the name of the item in your store you wish to modify.\n>")?
        }
    };

    if !store.exists(&item) {
        return Err(Error::ItemNotFound(item));
    }

    let password = input::read_secret(
        &format!("Enter new password for {}", item),
        Some(&format!("Retype new password for {}", item)),
    )?;

    store.update(&item, &password)?;
    eprintln!("Updated entry in the password store for {}.", item);
    Ok(())
}
