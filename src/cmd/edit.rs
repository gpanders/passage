use crate::error::Error;
use crate::input;
use crate::store::PasswordStore;

fn distance(a: &str, b: &str) -> u32 {
    let mut distance = (a.len() as isize - b.len() as isize).abs() as u32;
    for (i, j) in a.chars().zip(b.chars()) {
        if i != j {
            distance += 1;
        }
    }

    distance
}

pub fn edit(store: PasswordStore, item: Option<&str>) -> Result<(), Error> {
    let item = match item {
        Some(s) => s.to_string(),
        None => {
            input::read_input("Enter the name of the item in your store you wish to modify.\n>")?
        }
    };

    if !store.exists(&item) {
        match store.list()?.iter().map(|e| (e, distance(&item, &e))).min_by_key(|e| e.1) {
            Some((closest, distance)) => {
                if distance < 3 {
                    let prompt = format!("{} not found in the password store. Did you mean {} [Y/n]?", item, closest);
                    match input::read_input(&prompt) {
                        Ok(s) => if s.to_ascii_lowercase() == "y" {
                            return edit(store, Some(&s));
                        },
                        _ => {},
                    };
                }
            },
            _ => {},
        }
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
