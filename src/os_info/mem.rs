use std::io::{BufRead, BufReader};
use std::fs::File;
use std::path::Path;
use std::error::Error;
use std::process;
use std::result::Result;


pub fn get_mem() -> Box<str> {
    match mem_bar() {
        Ok(mem_bar) => mem_bar,
        Err(err) => {
           eprintln!("[Memory Error]: {}", err);
           process::exit(1);
        }
    } 
}

fn mem_bar() -> Result<Box<str>, Box<dyn Error>> {
    let (mem_total, mem_used) = ram_usage()?;
    let bar_nr = ((mem_used*10)/mem_total) as usize;
    let mut bar = ['[', '\u{25E6}', '\u{25E6}', '\u{25E6}', '\u{25E6}', '\u{25E6}', '\u{25E6}', '\u{25E6}', '\u{25E6}', '\u{25E6}', '\u{25E6}',']'];
    let mut counter = 1;
    while bar_nr >= counter {
        bar[counter] = '\u{2022}';
        counter += 1;
    }
    let bar = format!("{}M {} {}M", mem_used/1024, bar.iter().collect::<String>(), mem_total/1024);
    Ok(bar.into_boxed_str())
}

fn ram_usage() -> Result<(u32,u32), Box<dyn Error>> {
    let mut mem_free = 0;
    let mut mem_total = 0;
    let fb = BufReader::new(File::open(Path::new("/proc/meminfo"))?);
    for line in fb.lines() {
        if line.as_ref().unwrap().contains("MemFree") |
           line.as_ref().unwrap().contains("Buffers") |
           line.as_ref().unwrap().contains("Cached") |
           line.as_ref().unwrap().contains("SReclaimable") {
            mem_free += line?.split_whitespace().nth(1).unwrap().trim().parse::<u32>()?;
        } else if line.as_ref().unwrap().contains("MemTotal") {
                      mem_total += line?.split_whitespace().nth(1).unwrap().trim().parse::<u32>()?;
                  }
    }
    let mem_used = mem_total - mem_free;
    Ok((mem_total, mem_used))
}