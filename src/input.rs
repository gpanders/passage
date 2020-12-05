use std::io;
use std::io::prelude::*;

use crate::error::Error;

pub fn read_secret(prompt: &str, confirm: Option<&str>) -> Result<String, Error> {
    let input = rpassword::prompt_password_stdout(&format!("{}: ", prompt))?;

    match confirm {
        Some(prompt) => {
            if rpassword::prompt_password_stdout(&format!("{}: ", prompt))? != input {
                Err(Error::PasswordsDoNotMatch)
            } else {
                Ok(input)
            }
        }
        None => Ok(input),
    }
}

pub fn read_input(prompt: &str) -> Result<String, Error> {
    print!("{} ", prompt);
    io::stdout().flush()?;

    let mut ans = String::new();
    io::stdin().read_line(&mut ans)?;

    Ok(ans.trim_end().to_string())
}
