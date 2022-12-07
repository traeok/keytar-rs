use keyring::{Entry, Error};
use napi::bindgen_prelude::*;
use napi_derive::napi;

#[cfg(windows)]
use byteorder::{ByteOrder, LittleEndian};

#[cfg(windows)]
fn keyringEntry(service: &str, account: &str) -> Entry {
    let target = format!("{}/{}", &service, &account);
    Entry::new_with_target(&target, &target, account)
}

#[cfg(unix)]
fn keyringEntry(service: &str, account: &str) -> Entry {
    Entry::new(service, account)
}

#[cfg(windows)]
fn decodePassword(password: &str) -> String {
    let blob_u16: Vec<u16> = password.encode_utf16().collect();
    let mut blob = vec![0; blob_u16.len() * 2];
    LittleEndian::write_u16_into(&blob_u16, &mut blob);
    String::from_utf8_lossy(&blob).to_string()
}

#[cfg(unix)]
fn decodePassword(password: &str) -> String {
    password.to_string()
}

#[cfg(windows)]
fn encodePassword(password: &str) -> String {
    let blob = password.as_bytes();
    let mut blob_u16 = vec![0; blob.len() / 2];
    LittleEndian::read_u16_into(blob, &mut blob_u16);
    String::from_utf16_lossy(&blob_u16)
}

#[cfg(unix)]
fn encodePassword(password: &str) -> String {
    password.to_string()
}

#[napi]
fn getPassword(service: String, account: String) -> Result<Option<String>> {
    match keyringEntry(&service, &account).get_password() {
        Ok(password) => Ok(Some(decodePassword(&password))),
        Err(Error::NoEntry) => Ok(None),
        Err(e) => Err(napi::Error::new(Status::InvalidArg, e.to_string()))
    }
}

#[napi]
fn setPassword(service: String, account: String, password: String) -> Result<()> {
    match keyringEntry(&service, &account).set_password(&encodePassword(&password)) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keyring() {
        let testService = "test";
        let testAccount = "user";
        let testPassword = "password💖";
        setPassword(testService.to_string(), testAccount.to_string(), testPassword.to_string()).unwrap();
        assert_eq!(getPassword(testService.to_string(), testAccount.to_string()).unwrap(), Some(testPassword.to_string()));
        assert_eq!(deletePassword(testService.to_string(), testAccount.to_string()).unwrap(), true);
        assert_eq!(getPassword(testService.to_string(), testAccount.to_string()).unwrap(), None);
    }
}
