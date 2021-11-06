use std::error::Error;
use std::result;
use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::process;
use std::fmt;

pub struct Uptime(u32, u32); 

impl fmt::Display for Uptime {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       write!(f, "{}h {}m", self.0, self.1)
   } 
}

pub fn get_uptime() -> Uptime {
    match uptime() {
        Ok(uptime) => uptime,
        Err(err) => {
           eprintln!("[Uptime Error]: {}", err);
           process::exit(1);
        }
    } 
}

fn uptime() -> result::Result<Uptime, Box<dyn Error>> {
    let mut fb = BufReader::new(File::open(Path::new("/proc/uptime"))?);
    let mut first_line = String::new();
    let _ = fb.read_line(&mut first_line);

    let uptime = first_line.split_whitespace().next().unwrap().trim().parse::<f64>()?;

    let (mut minutes,mut hours) = (uptime as u32, 0);

    if minutes > 3600 {
        while minutes > 3600 {
            minutes -= 3600;
            hours += 1;
        }

        minutes /= 60;

    } else {
        minutes /= 60;
    }

    Ok(Uptime(hours, minutes))    
}