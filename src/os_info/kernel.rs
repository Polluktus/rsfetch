use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::error::Error;
use std::result::Result;
use std::process;

pub fn get_kernel() -> Box<str> {
    match kernel() {
        Ok(kernel) => kernel,
        Err(err) => {
           eprintln!("[Kernel Error]: {}", err);
           process::exit(1);
        }
    } 
}

fn kernel() -> Result<Box<str>, Box<dyn Error>> {

    let mut fb = BufReader::new(File::open(Path::new("/proc/version"))?);
    let mut first_line = String::new();
    let _ = fb.read_line(&mut first_line);

    let version = first_line.split_whitespace().nth(2).unwrap().to_string();
    Ok(version.into_boxed_str())
}