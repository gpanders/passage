use crate::error::Error;
use crate::key;
use secrecy::ExposeSecret;

pub fn key(secret: bool) -> Result<(), Error> {
    let key = key::read_secret_key(key::secret_key_path())?;
    if secret {
        println!("{}", key.to_string().expose_secret());
    } else {
        println!("{}", key.to_public());
    }
    Ok(())
}
