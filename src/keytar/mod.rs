pub mod win;

#[cfg(target_os = "windows")]
pub use win::{delete_password, find_credentials, find_password, get_password, set_password};
