use keyring::{Entry, Error};
use napi::bindgen_prelude::*;
use napi_derive::napi;

#[cfg(windows)]
fn keyringEntry(service: &str, account: &str) -> Entry {
    let target = format!("{}/{}", &service, &account);
    Entry::new_with_target(&target, &target, &account)
}

#[cfg(unix)]
fn keyringEntry(service: &str, account: &str) -> Entry {
    Entry::new(&service, &account)
}

#[napi]
fn getPassword(service: String, account: String) -> Result<Option<String>> {
    match keyringEntry(&service, &account).get_password() {
        Ok(password) => Ok(Some(password)),
        Err(Error::NoEntry) => Ok(None),
        Err(e) => Err(napi::Error::new(Status::InvalidArg, e.to_string()))
    }
}

#[napi]
fn setPassword(service: String, account: String, password: String) -> Result<()> {
    match keyringEntry(&service, &account).set_password(&password) {
        Ok(()) => Ok(()),
        Err(e) => Err(napi::Error::new(Status::InvalidArg, e.to_string()))
    }
}

#[napi]
fn deletePassword(service: String, account: String) -> Result<bool> {
    match keyringEntry(&service, &account).delete_password() {
        Ok(_) => Ok(true),
        Err(Error::NoEntry) => Ok(false),
        Err(e) => Err(napi::Error::new(Status::InvalidArg, e.to_string()))
    }
}
