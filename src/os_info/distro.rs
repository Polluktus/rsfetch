use std::result::Result;
use std::fs::File;
use std::io::{BufReader, BufRead, Error};
use std::path::Path;
use std::process;

pub fn distro_name() -> Box<str> {
    match parse_file() {
        Ok(name) => name,
        Err(err) => {
            eprintln!("[Distro Error]: {}", err);
            process::exit(1);
        }
    }
}

fn get_file() -> Result<BufReader<File>, Box<Error>> {
   
    let f =  File::open(
       Path::new("/etc/os-release")
    )?;

    let buffer = BufReader::new(f);

    Ok(buffer)
}

fn parse_file() -> Result<Box<str>, Box<Error>> {
    let buff_file = get_file()?;

    let mut name = String::new();

    for line in buff_file.lines() {
        let line = line?;

        if line.starts_with("NAME") {
            name = line.chars().skip(5).collect();
            break;
        }
    }        

    if name.starts_with('"') {
        name = String::from(&name[1..name.len()-1])
    }

    //let name = get_name(&name_line);      //maybe its useless since we know os-release syntax

    if name.is_empty() {
        return Ok(String::from("Linux").into_boxed_str()); //os-release may not contain any name
    }

    Ok(name.into_boxed_str())
}

/*fn get_name(name_line: &str) -> &str {    
    let bytes = name_line.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b'=' {
            return &name_line[i+1..];
        }
    }
    &name_line
}*/