use std::env;
use std::env::VarError;
use std::process;
use std::result::Result;

pub fn get_shell() -> Box<str> {
    match shell() {
        Ok(shell) => shell,
        Err(err) => {
           eprintln!("[Shell Error]: {}", err);
           process::exit(1);
        }
    } 
}

fn shell() -> Result<Box<str>, Box<VarError>> {
    let shell = env::var("SHELL")?.chars().skip(5).collect::<String>();     //skips "/bin/"
    Ok(shell.into_boxed_str())
}