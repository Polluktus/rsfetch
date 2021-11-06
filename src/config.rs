pub const USER_ICON: char = '\u{F2C0}';         //     they must be Unicode (Big-Endian), syntax \u{utf_code}
pub const DESKTOP_ICON: char = '\u{F823}';      //
pub const LINUX_ICON: char = '\u{E712}';        //
pub const TIME_ICON: char = '\u{E386}';         //
pub const TERMINAL_ICON: char = '\u{F489}';     //
pub const PKG_ICON: char = '\u{F8D6}';          //

pub const RESET: &str = "\x1b[0m";              //default terminal colors, you can change them 
pub const BLACK: &str = "\x1b[30m";             //for e.g. some specific rgb color
pub const RED: &str = "\x1b[31m";
pub const GREEN: &str = "\x1b[32m";
pub const YELLOW: &str = "\x1b[33m";
pub const BLUE: &str = "\x1b[34m";
pub const MAGENTA: &str = "\x1b[35m";
pub const CYAN: &str = "\x1b[36m";

pub const SHOW_COLORS: bool = true;             //show colour characters [true or false]
pub const COLOUR_CHARACTER: char = '●';         

/*
you can use characters like 
▀ ▁ ▂ ▃ ▄ ▅ ▆ ▇ █ ▉ ▊ ▋ ▌ ▍ ▎ ▏ █ ▄ ▀ ● 
or something else as you like
*/
