use std::process::{Command, Stdio};
use std::error::Error;
use std::result::Result;
use std::sync::mpsc::Sender;

pub fn get_packages(distro: &str, tx: Sender<u32>) {
    match distro {
        "Gentoo" => {
            let pkg = portage_packages().unwrap_or(0);
            tx.send(pkg).unwrap();
        },
        "Arch Linux" | "Manjaro Linux" | "Artix Linux" => {
            let pkg = pacman_packages().unwrap_or(0);
            tx.send(pkg).unwrap();
        },
        "Ubuntu" | "Debian GNU/Linux" | "Linux Mint"| "elementary OS" => {
            let pkg = apt_packages().unwrap_or(0);
            tx.send(pkg).unwrap();
        },
        "Fedora Linux" | "Rocky Linux" => {
            let pkg = rpm_packages().unwrap_or(0);
            tx.send(pkg).unwrap();
        },
        "void" => {
            let pkg = xbps_packages().unwrap_or(0);
            tx.send(pkg).unwrap();
        },
        _ => tx.send(0).unwrap()
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

    let pkg = String::from_utf8_lossy(&count_pkg).trim().parse::<u32>()?;
    Ok(pkg)
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

    let pkg = String::from_utf8_lossy(&count_pkg).trim().parse::<u32>()?;
    Ok(pkg)
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

    let pkg = String::from_utf8_lossy(&count_pkg).trim().parse::<u32>()?;
    Ok(pkg)
}

fn rpm_packages() -> Result<u32, Box<dyn Error>> {

    let count_pkg = Command::new("bash")
                                .arg("-c")
                                .arg("[[ $(which sqlite3 2> /dev/null) && $? -ne 1 ]] && ((sqlite3 /var/lib/rpm/rpmdb.sqlite \"select COUNT(*) from Name\") || (sqlite3 /var/cache/dnf/packages.db \"SELECT count(pkg) FROM installed\")) || rpm -qa | wc -l")
                                .output()?
                                .stdout;


    let pkg = String::from_utf8_lossy(&count_pkg).trim().parse::<u32>()?;
    Ok(pkg)
}

fn xbps_packages() -> Result<u32, Box<dyn Error>> {

    let packages = Command::new("xbps-query")
                                .arg("-l")
                                .stdout(Stdio::piped())
                                .spawn()?
                                .stdout
                                .unwrap();

    let count_pkg = Command::new("wc")
                            .arg("-l")
                            .stdin(packages)
                            .output()?
                            .stdout;

    let pkg = String::from_utf8_lossy(&count_pkg).trim().parse::<u32>()?;
    Ok(pkg)
}
