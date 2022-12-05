mod win;

#[cfg(target_os = "windows")]
use win::{delete_password, find_credentials, find_password, get_password, set_password};
