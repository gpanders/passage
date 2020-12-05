use crate::error::Error;
use crate::input;
use crate::key;

pub fn lock() -> Result<(), Error> {
    let passphrase = input::read_secret("Enter passphrase", Some("Confirm passphrase"))?;
    key::encrypt_secret_key(key::secret_key_path(), &passphrase)?;

    println!("Password store locked.");
    Ok(())
}

pub fn unlock() -> Result<(), Error> {
    key::decrypt_secret_key(key::secret_key_path(), None)?;
    println!("Password store unlocked.");
    Ok(())
}
