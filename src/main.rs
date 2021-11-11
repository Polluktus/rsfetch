mod os_info;
mod config;
use config::*;
use os_info::Uptime;

struct System {
    user: Box<str>,
    distro: Box<str>,
    kernel: Box<str>,
    uptime: Uptime,
    shell: Box<str>,
    packages: u32
}

impl System {
    fn new() -> System {
        let distro = os_info::distro_name();
        let packages = os_info::get_packages(&distro);
        System {
            user: os_info::get_user(),
            distro,
            kernel: os_info::get_kernel(),
            uptime: os_info::get_uptime(),
            shell: os_info::get_shell(),
            packages
        }
    }
}

fn main() {
    let system = System::new();

    match &*system.distro {
        "Gentoo" => {    
            println!("{}   .-----.{}
{} .`    _  `.      {} {}{}
{} `.   (_)   `.    {} {}{}
{}   `.        /    {} {}{}
{}  .`       .`     {} {}{}
{} /       .`       {} {}{}
{} \\____.-`         {} {}{}",
MAGENTA, RESET,
MAGENTA, USER_ICON, RESET, system.user,
MAGENTA, DESKTOP_ICON, RESET, system.distro,
MAGENTA, LINUX_ICON, RESET, system.kernel,
MAGENTA, TIME_ICON, RESET, system.uptime,
MAGENTA, TERMINAL_ICON, RESET, system.shell,
MAGENTA, PKG_ICON, RESET, system.packages
);
            colour_dots();
        }
        "Arch Linux" => {
            println!("{}        /\\        {} {}{}
{}       /  \\       {} {}{}
{}      /\\   \\      {} {}{}
{}     /  __  \\     {} {}{}
{}    /  (  )  \\    {} {}{}
{}   / __|  |__\\\\   {} {}{}
{}  /.`        `.\\{}",
BLUE, USER_ICON, RESET, system.user,
BLUE, DESKTOP_ICON, RESET, system.distro,
BLUE, LINUX_ICON, RESET, system.kernel,
BLUE, TIME_ICON, RESET, system.uptime,
BLUE, TERMINAL_ICON, RESET, system.shell,
BLUE, PKG_ICON, RESET, system.packages,
BLUE, RESET
);
            colour_dots();
        }
        "Manjaro Linux" => {
            println!("{}  ||||||||| ||||       {} {}{}
{}  ||||||||| ||||       {} {}{}
{}  ||||      ||||       {} {}{}
{}  |||| |||| ||||       {} {}{}
{}  |||| |||| ||||       {} {}{}
{}  |||| |||| ||||       {} {}{}
{}  |||| |||| ||||{}",
GREEN, USER_ICON, RESET, system.user,
GREEN, DESKTOP_ICON, RESET, system.distro,
GREEN, LINUX_ICON, RESET, system.kernel,
GREEN, TIME_ICON, RESET, system.uptime,
GREEN, TERMINAL_ICON, RESET, system.shell,
GREEN, PKG_ICON, RESET, system.packages,
GREEN, RESET
);
            colour_dots();
        }
        "Ubuntu" => {
            println!("{}           _     {} {}{}
{}       ---(_)    {} {}{}
{}   _/  ---  \\    {} {}{}
{}  (_) |   |      {} {}{}
{}    \\  --- _/    {} {}{}
{}       ---(_)    {} {}{}",
YELLOW, USER_ICON, RESET, system.user,
YELLOW, DESKTOP_ICON, RESET, system.distro,
YELLOW, LINUX_ICON, RESET, system.kernel,
YELLOW, TIME_ICON, RESET, system.uptime,
YELLOW, TERMINAL_ICON, RESET, system.shell,
YELLOW, PKG_ICON, RESET, system.packages,
);
            colour_dots()
        }
        "Artix Linux" => {
            println!("{}        /\\        {} {}{}
{}       /  \\       {} {}{}
{}      /`'.,\\      {} {}{}
{}     /     ',     {} {}{}
{}    /      ,`\\    {} {}{}
{}   /   ,.'`.  \\   {} {}{}
{}  /.,'`     `'.\\{}",
BLUE, USER_ICON, RESET, system.user,
BLUE, DESKTOP_ICON, RESET, system.distro,
BLUE, LINUX_ICON, RESET, system.kernel,
BLUE, TIME_ICON, RESET, system.uptime,
BLUE, TERMINAL_ICON, RESET, system.shell,
BLUE, PKG_ICON, RESET, system.packages,
BLUE, RESET
);
            colour_dots();
        }
        "Debian GNU/Linux" => {
            println!("{}     ,---._     {} {}{}
{}   /`  __  \\    {} {}{}
{}  |   /    |    {} {}{}
{}  |   `.__.`    {} {}{}
{}   \\            {} {}{}
{}    `-,_        {} {}{}",
RED, USER_ICON, RESET, system.user,
RED, DESKTOP_ICON, RESET, system.distro,
RED, LINUX_ICON, RESET, system.kernel,
RED, TIME_ICON, RESET, system.uptime,
RED, TERMINAL_ICON, RESET, system.shell,
RED, PKG_ICON, RESET, system.packages,
);
            colour_dots();
        }
        "Linux Mint" => {
            println!("{}   _____________      {}
{}  |_            \\     {} {}{}
{}    |  {}| _____{}  |     {} {}{}
{}    |  {}| | | |{}  |     {} {}{}
{}    |  {}| | | |{}  |     {} {}{}
{}    |  {}\\__{}___/  |     {} {}{}
{}    \\___________/     {} {}{}",
GREEN, RESET,
GREEN, USER_ICON, RESET, system.user,
GREEN, RESET , GREEN ,DESKTOP_ICON, RESET, system.distro,
GREEN, RESET , GREEN ,LINUX_ICON, RESET, system.kernel,
GREEN, RESET, GREEN ,TIME_ICON, RESET, system.uptime,
GREEN, RESET, GREEN ,TERMINAL_ICON, RESET, system.shell,
GREEN, PKG_ICON, RESET, system.packages
);
            colour_dots();
        }
        "Fedora" => {
            println!("        _____
       /   __){}\\     {} {}{}
       |  /  {}\\ \\    {} {}{}
{}    __{}_|  |_{}_/ /    {} {}{}
{}   / {}(_    _){}_/     {} {}{}
{}  / /  {}|  |         {}{} {}{}
{}  \\ \\{}__/  |         {}{} {}{}
{}   \\{}(_____/",
BLUE, USER_ICON, RESET, system.user,
BLUE, DESKTOP_ICON, RESET, system.distro,
BLUE, RESET, BLUE, LINUX_ICON, RESET, system.kernel,
BLUE, RESET, BLUE, TIME_ICON, RESET, system.uptime,
BLUE, RESET, BLUE, TERMINAL_ICON, RESET, system.shell,
BLUE, RESET, BLUE, PKG_ICON, RESET, system.packages,
BLUE, RESET
);
            colour_dots();
        }
        _ => {
            println!("{}      ___     {}
{}     ({}..{} \\        {}{} {}
{}     ({}<>{} |        {}{} {}
{}    /{}/  \\ {}\\       {}{} {}
{}   ( {}|  |{} /|      {}{} {}
{}  _{}/\\ {}__){}/{}_{})      {}{} {}
{}  \\/{}-____{}\\/       {}{} {}
",
BLACK, RESET,
BLACK, RESET, BLACK, RESET, USER_ICON, system.user,
BLACK, YELLOW, BLACK, RESET, DESKTOP_ICON, system.distro,
BLACK, RESET, BLACK, RESET, LINUX_ICON, system.kernel,
BLACK, RESET, BLACK, RESET, TIME_ICON, system.uptime,
YELLOW, BLACK, RESET, BLACK, YELLOW, BLACK, RESET, TERMINAL_ICON, system.shell,
YELLOW, BLACK, YELLOW, RESET, PKG_ICON, system.packages
);
            colour_dots();
        } 
    }
}

fn colour_dots() {

    if !SHOW_COLORS {
        return;
    }

    print!("\n    ");
    for i in 30..38 {
        print!("  \x1b[{}m{}\x1b[0m", i, COLOUR_CHARACTER);
    }
    print!("\n    ");
    for i in 90..98 {
        print!("  \x1b[{}m{}\x1b[0m", i, COLOUR_CHARACTER);
    }
    println!();
}
