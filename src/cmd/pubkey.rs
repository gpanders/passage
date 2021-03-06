use crate::error::Error;
use crate::key;

pub fn pubkey() -> Result<(), Error> {
    let key = key::read_secret_key(key::secret_key_path())?;
    println!("{}", key.to_public());
    Ok(())
}
