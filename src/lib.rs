use keyring;
use napi::{Env, Error, JsBoolean, JsString, Result, Task};
use napi::bindgen_prelude::AsyncTask;
use napi_derive::napi;

#[cfg(windows)]
use byteorder::{ByteOrder, LittleEndian};

#[cfg(windows)]
fn credential(service: &str, account: &str) -> keyring::Entry {
    let target = format!("{}/{}", &service, &account);
    keyring::Entry::new_with_target(&target, &target, account)
}

#[cfg(unix)]
fn credential(service: &str, account: &str) -> keyring::Entry {
    keyring::Entry::new(service, account)
}

#[cfg(windows)]
fn decode_utf16(password: &str) -> String {
    let blob_u16: Vec<u16> = password.encode_utf16().collect();
    let mut blob = vec![0; blob_u16.len() * 2];
    LittleEndian::write_u16_into(&blob_u16, &mut blob);
    String::from_utf8_lossy(&blob).to_string()
}

#[cfg(unix)]
fn decode_utf16(password: &str) -> String {
    password.to_string()
}

#[cfg(windows)]
fn encode_utf16(password: &str) -> String {
    let blob = password.as_bytes();
    let mut blob_u16 = vec![0; blob.len() / 2];
    LittleEndian::read_u16_into(blob, &mut blob_u16);
    String::from_utf16_lossy(&blob_u16)
}

#[cfg(unix)]
fn encode_utf16(password: &str) -> String {
    password.to_string()
}

pub struct GetPassword {
    pub service: String,
    pub account: String,
}

pub struct SetPassword {
    pub service: String,
    pub account: String,
    pub password: String,
}

pub struct DeletePassword {
    pub service: String,
    pub account: String,
}

#[napi]
impl Task for GetPassword {
    type Output = Option<String>;
    type JsValue = Option<JsString>;

    fn compute(&mut self) -> Result<Self::Output> {
        match credential(&self.service, &self.account).get_password() {
            Ok(password) => Ok(Some(decode_utf16(&password))),
            Err(keyring::Error::NoEntry) => Ok(None),
            Err(e) => Err(napi::Error::from_reason(e.to_string()))
        }
    }

    fn resolve(&mut self, env: Env, output: Self::Output) -> Result<Self::JsValue> {
        output.map(|password| env.create_string(password.as_str())).transpose()
    }

    fn reject(&mut self, env: Env, err: Error) -> Result<Self::JsValue> {
        Err(err)
    }
}

#[napi]
impl Task for SetPassword {
    type Output = ();
    type JsValue = ();

    fn compute(&mut self) -> Result<Self::Output> {
        match credential(&self.service, &self.account).set_password(&encode_utf16(&self.password)) {
            Ok(()) => Ok(()),
            Err(e) => Err(napi::Error::from_reason(e.to_string()))
        }
    }

    fn resolve(&mut self, env: Env, output: Self::Output) -> Result<Self::JsValue> {
        Ok(())
    }

    fn reject(&mut self, env: Env, err: Error) -> Result<Self::JsValue> {
        Err(err)
    }
}

#[napi]
impl Task for DeletePassword {
    type Output = bool;
    type JsValue = JsBoolean;

    fn compute(&mut self) -> Result<Self::Output> {
        match credential(&self.service, &self.account).delete_password() {
            Ok(_) => Ok(true),
            Err(keyring::Error::NoEntry) => Ok(false),
            Err(e) => Err(napi::Error::from_reason(e.to_string()))
        }
    }

    fn resolve(&mut self, env: Env, output: Self::Output) -> Result<Self::JsValue> {
        env.get_boolean(output)
    }

    fn reject(&mut self, env: Env, err: Error) -> Result<Self::JsValue> {
        Err(err)
    }
}

#[napi]
fn getPassword(service: String, account: String) -> AsyncTask<GetPassword> {
    AsyncTask::new(GetPassword { service, account })
}

#[napi]
fn setPassword(service: String, account: String, password: String) -> AsyncTask<SetPassword> {
    AsyncTask::new(SetPassword { service, account, password })
}

#[napi]
fn deletePassword(service: String, account: String) -> AsyncTask<DeletePassword> {
    AsyncTask::new(DeletePassword { service, account })
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_keyring() {
//         let testService = "test";
//         let testAccount = "user";
//         let testPassword = "password💖";
//         setPassword(testService.to_string(), testAccount.to_string(), testPassword.to_string()).unwrap();
//         assert_eq!(getPassword(testService.to_string(), testAccount.to_string()).unwrap(), Some(testPassword.to_string()));
//         assert_eq!(deletePassword(testService.to_string(), testAccount.to_string()).unwrap(), true);
//         assert_eq!(getPassword(testService.to_string(), testAccount.to_string()).unwrap(), None);
//     }
// }
