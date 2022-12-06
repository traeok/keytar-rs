use libsecret::*;
use libsecret_sys::*;

// TODO
fn set_password(service: String, account: String, password: String) {
  secret_password_store_sync()
}
