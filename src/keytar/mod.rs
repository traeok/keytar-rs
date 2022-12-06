pub mod mac;
pub mod unix;
pub mod win;

#[cfg(target_os = "windows")]
pub use win::{delete_password, find_credentials, find_password, get_password, set_password};

#[cfg(target_os = "macos")]
pub use mac::{delete_password, find_credentials, find_password, get_password, set_password};

#[cfg(target_os = "linux")]
pub use unix::{delete_password, find_credentials, find_password, get_password, set_password};
