use std::env;
use std::env::VarError;
use std::process;
use std::result::Result;

pub fn get_user() -> Box<str> {
    match user() {
        Ok(user) => user,
        Err(err) => {
           eprintln!("[User Error]: {}", err);
           process::exit(1);
        }
    } 
}

fn user() -> Result<Box<str>, Box<VarError>> {
    let shell = env::var("USER")?;
    Ok(shell.into_boxed_str())
}