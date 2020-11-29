use age::cli_common::read_secret;
use passage::Error;

pub fn lock() -> Result<(), Error> {
    let passphrase = read_secret("Passphrase", "Passphrase", None)?;
    passage::encrypt_secret_key(passage::secret_key_path(), &passphrase)?;

    println!("Password store locked.");
    Ok(())
}

pub fn unlock() -> Result<(), Error> {
    let passphrase = read_secret("Passphrase", "Passphrase", None)?;
    passage::decrypt_secret_key(passage::secret_key_path(), &passphrase)?;

    println!("Password store unlocked.");
    Ok(())
}
