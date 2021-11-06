mod distro;
pub use self::distro::distro_name;

mod packages;
pub use self::packages::get_packages;

mod shell;
pub use self::shell::get_shell;

mod user;
pub use self::user::get_user;

mod kernel;
pub use self::kernel::get_kernel;

mod uptime;
pub use self::uptime::{get_uptime, Uptime};