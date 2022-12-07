use std::ffi::c_void;
use std::result::Result;
use windows::{core::*, Win32::Foundation::*, Win32::Security::Credentials::*};

use crate::keytar::error::Error;

pub fn set_password(
  service: &String,
  account: &String,
  password: &mut String,
) -> Result<bool, Error> {
  let mut cred: CREDENTIALW = CREDENTIALW::default();
  cred.Type = CRED_TYPE_GENERIC;
  let mut target_bytes: Vec<u16> = format!("{}/{}", service, account).encode_utf16().collect();
  target_bytes.push(0);
  cred.TargetName = PWSTR::from_raw(target_bytes.as_mut_ptr());
  let mut username_bytes: Vec<u16> = account.encode_utf16().collect();
  username_bytes.push(0);
  cred.UserName = PWSTR::from_raw(username_bytes.as_mut_ptr());
  cred.CredentialBlobSize = password.len() as u32;
  cred.CredentialBlob = password.as_mut_ptr();
  cred.Persist = CRED_PERSIST_ENTERPRISE;
  unsafe { Ok(bool::from(CredWriteW(&cred, 0))) }
}

pub fn get_password(service: &String, account: &String) -> Result<String, Error> {
  let mut cred: *mut CREDENTIALW = std::ptr::null_mut::<CREDENTIALW>();
  let mut target_name: Vec<u16> = format!("{}/{}", service, account).encode_utf16().collect();
  target_name.push(0);

  let read_result: bool;
  unsafe {
    read_result = bool::from(CredReadW(
      PCWSTR::from_raw(target_name.as_mut_ptr()),
      CRED_TYPE_GENERIC.0,
      0,
      &mut cred,
    ));
  }

  if !read_result {
    let code: WIN32_ERROR;
    unsafe {
      code = GetLastError();
    }

    return Err(Error::from_win(code.0));
  }

  let mut pw_bytes: Vec<u8> = Vec::new();
  unsafe {
    let pw_len = (*cred).CredentialBlobSize as usize;
    pw_bytes.reserve(pw_len);

    CredFree(cred as *const c_void);
    Ok(String::from(
      std::str::from_utf8(std::slice::from_raw_parts((*cred).CredentialBlob, pw_len)).unwrap(),
    ))
  }
}

pub fn delete_password(service: &String, account: &String) -> Result<bool, Error> {
  let mut target_name: Vec<u16> = format!("{}/{}", service, account).encode_utf16().collect();
  target_name.push(0);

  let delete_result: bool;
  unsafe {
    delete_result = bool::from(CredDeleteW(
      PCWSTR::from_raw(target_name.as_mut_ptr()),
      CRED_TYPE_GENERIC.0,
      0,
    ));
  }

  if !delete_result {
    let code: WIN32_ERROR;
    unsafe {
      code = GetLastError();
    }
    if code == ERROR_NOT_FOUND {
      // If we are trying to delete a credential that doesn't exist,
      // we didn't actually delete the password
      return Ok(false);
    }

    return Err(Error::from_win(code.0));
  }

  Ok(true)
}

pub fn find_password(service: String) -> Result<String, Error> {
  let mut filter: Vec<u16> = format!("{}*", service).encode_utf16().collect();

  let mut count: u32 = 0;
  let mut creds: *mut *mut CREDENTIALW = std::ptr::null_mut::<*mut CREDENTIALW>();

  let result: bool;
  unsafe {
    result = bool::from(CredEnumerateW(
      PCWSTR::from_raw(filter.as_mut_ptr()),
      CRED_ENUMERATE_FLAGS(0),
      &mut count,
      &mut creds as *mut *mut *mut CREDENTIALW,
    ));
  }

  if !result {
    let code: WIN32_ERROR;
    unsafe {
      code = GetLastError();
    }
    if code == ERROR_NOT_FOUND {
      return Ok(String::default());
    }

    return Err(Error::from_win(code.0));
  }

  let cred: *const CREDENTIALW;
  unsafe {
    cred = *creds.offset(0);
    let size = (*cred).CredentialBlobSize as usize;
    CredFree(creds as *const c_void);

    Ok(String::from(
      std::str::from_utf8(std::slice::from_raw_parts((*cred).CredentialBlob, size)).unwrap(),
    ))
  }
}

pub fn find_credentials(
  service: &String,
  credentials: &mut Vec<(String, String)>,
) -> Result<bool, Error> {
  let mut filter_bytes = format!("{}*", service).encode_utf16().collect::<Vec<u16>>();
  filter_bytes.push(0);
  let filter = PCWSTR::from_raw(filter_bytes.as_mut_ptr());

  let mut count: u32 = 0;
  let mut creds: *mut *mut CREDENTIALW = std::ptr::null_mut::<*mut CREDENTIALW>();

  let result: bool;
  unsafe {
    result = bool::from(CredEnumerateW(
      filter,
      CRED_ENUMERATE_FLAGS(0),
      &mut count,
      &mut creds as *mut *mut *mut CREDENTIALW,
    ));
  }

  if !result {
    let code: WIN32_ERROR;
    unsafe {
      code = GetLastError();
    }
    if code == ERROR_NOT_FOUND {
      return Ok(false);
    }

    return Err(Error::from_win(code.0));
  }

  for i in 0..count {
    let cred: &CREDENTIALW;
    unsafe {
      cred = &**creds.offset(i as isize);
    }

    if cred.UserName.is_null() || cred.CredentialBlobSize == 0 {
      continue;
    }

    let password: String;
    unsafe {
      password = String::from(
        std::str::from_utf8(std::slice::from_raw_parts(
          cred.CredentialBlob,
          cred.CredentialBlobSize as usize,
        ))
        .unwrap(),
      );
    }

    let username: String;
    unsafe {
      username = cred.UserName.to_string().unwrap();
    }
    credentials.push((username, password));
  }

  unsafe {
    CredFree(creds as *const c_void);
  }

  Ok(true)
}
