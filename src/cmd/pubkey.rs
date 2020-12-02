use passage::Error;

pub fn pubkey() -> Result<(), Error> {
    let key = passage::read_secret_key(passage::secret_key_path())?;
    println!("{}", key.to_public());
    Ok(())
}
