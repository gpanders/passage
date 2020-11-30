use passage::Error;

pub fn lock() -> Result<(), Error> {
    let passphrase = passage::read_secret("Enter passphrase", Some("Confirm passphrase"))?;
    passage::encrypt_secret_key(passage::secret_key_path(), &passphrase)?;

    println!("Password store locked.");
    Ok(())
}

pub fn unlock() -> Result<(), Error> {
    passage::decrypt_secret_key(passage::secret_key_path(), None)?;
    println!("Password store unlocked.");
    Ok(())
}
