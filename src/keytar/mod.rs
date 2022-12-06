pub mod error;

#[cfg(target_os = "windows")]
pub mod win;
pub use win::{delete_password, find_credentials, find_password, get_password, set_password};

// TODO: Not ideal to repeat cfg check, maybe update?
#[cfg(target_os = "macos")]
pub mod mac;
#[cfg(target_os = "macos")]
pub use mac::{delete_password, find_credentials, find_password, get_password, set_password};

#[cfg(target_os = "linux")]
pub mod unix;
#[cfg(target_os = "linux")]
pub use unix::{delete_password, find_credentials, find_password, get_password, set_password};
