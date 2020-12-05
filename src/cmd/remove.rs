use crate::error::Error;
use crate::input;
use crate::store::PasswordStore;

pub fn remove(store: PasswordStore, item: Option<&str>, force: bool) -> Result<(), Error> {
    let item = match item {
        Some(s) => s.to_string(),
        None => {
            input::read_input("Enter the name of the item you wish to remove from your store.\n>")?
        }
    };

    if !store.exists(&item) {
        return Err(Error::ItemNotFound(item));
    }

    if !force {
        let ans = input::read_input(&format!("Delete {}? [y/N]", item))?.to_lowercase();
        if ans != "y" && ans != "yes" {
            return Ok(());
        }
    }

    store.delete(&item)?;
    eprintln!("Removed {} from the password store.", item);
    Ok(())
}
