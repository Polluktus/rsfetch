use std::process::{Command, Stdio};
use std::error::Error;
use std::result::Result;

pub fn get_packages(distro: &str) -> u32 {
    match distro {
        "Gentoo" => portage_packages().unwrap_or(0),
        "Arch Linux" | "Manjaro Linux" | "Artix Linux" => pacman_packages().unwrap_or(0),
        "Ubuntu" | "Debian GNU/Linux" | "Linux Mint" => apt_packages().unwrap_or(0),
        _ => 0
    }
}

fn portage_packages() -> Result<u32, Box<dyn Error>> {

    let packages = Command::new("qlist")
                                .arg("-I")
                                .stdout(Stdio::piped())
                                .spawn()?
                                .stdout
                                .unwrap();

    let count_pkg = Command::new("wc")
                            .arg("-l")
                            .stdin(packages)
                            .output()?
                            .stdout;

    let pkg_string = String::from_utf8_lossy(&count_pkg);
    let final_num = pkg_string.trim().parse::<u32>()?;
    Ok(final_num)
}

fn pacman_packages() -> Result<u32, Box<dyn Error>> {

    let packages = Command::new("pacman")
                                .arg("-Q")
                                .arg("-q")
                                .stdout(Stdio::piped())
                                .spawn()?
                                .stdout
                                .unwrap();

    let count_pkg = Command::new("wc")
                            .arg("-l")
                            .stdin(packages)
                            .output()?
                            .stdout;

    let pkg_string = String::from_utf8_lossy(&count_pkg);
    let final_num = pkg_string.trim().parse::<u32>()?;
    Ok(final_num)
}

fn apt_packages() -> Result<u32, Box<dyn Error>> {

    let packages = Command::new("dpkg")
                                .arg("-l")
                                .stdout(Stdio::piped())
                                .spawn()?
                                .stdout
                                .unwrap();

    let tail = Command::new("tail")
                        .arg("-n+6")
                        .stdout(Stdio::piped())
                        .stdin(packages)
                        .spawn()?
                        .stdout
                        .unwrap();

    let count_pkg = Command::new("wc")
                            .arg("-l")
                            .stdin(tail)
                            .output()?
                            .stdout;

    let pkg_string = String::from_utf8_lossy(&count_pkg);
    let final_num = pkg_string.trim().parse::<u32>()?;
    Ok(final_num)
}