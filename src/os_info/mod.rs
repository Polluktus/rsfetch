mod distro;
pub use self::distro::distro_name;

mod packages;
pub use self::packages::get_packages;

mod mem;
pub use self::mem::get_mem;

mod user;
pub use self::user::get_user;

mod kernel;
pub use self::kernel::get_kernel;

mod uptime;
pub use self::uptime::{get_uptime, Uptime};