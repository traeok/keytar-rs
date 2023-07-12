use super::error::KeytarError;
use std::ffi::{c_void, OsStr};
use std::os::windows::prelude::OsStrExt;
use std::result::Result;
use windows_sys::{
  core::*,
  Win32::Foundation::*,
  Win32::Security::Credentials::*,
  Win32::System::Diagnostics::Debug::{
    FormatMessageW, FORMAT_MESSAGE_ALLOCATE_BUFFER, FORMAT_MESSAGE_FROM_SYSTEM,
    FORMAT_MESSAGE_IGNORE_INSERTS,
  },
  Win32::System::WindowsProgramming::uaw_wcslen,
};

impl From<WIN32_ERROR> for KeytarError {
  fn from(error: WIN32_ERROR) -> Self {
    KeytarError::Os(win32_error_as_string(error))
  }
}

/**
 * Helper function to convert the last Win32 error into a human-readable error message.
 * Returns: A String containing the error message.
 */
fn win32_error_as_string(error: WIN32_ERROR) -> String {
  let buffer: PWSTR = std::ptr::null_mut();

  // https://github.com/microsoft/windows-rs/blob/master/crates/libs/core/src/hresult.rs#L96
  let as_hresult = if error == 0 {
    0
  } else {
    (error & 0x0000_FFFF) | (7 << 16) | 0x8000_0000
  } as _;
  unsafe {
    let size = FormatMessageW(
      FORMAT_MESSAGE_ALLOCATE_BUFFER | FORMAT_MESSAGE_FROM_SYSTEM | FORMAT_MESSAGE_IGNORE_INSERTS,
      as_hresult as *const c_void,
      as_hresult,
      0,
      buffer,
      0,
      std::ptr::null()
    );

    String::from_utf16(std::slice::from_raw_parts(
      buffer,
      size as usize
    )).unwrap_or("No error details available.".to_string())
  }
}

/**
 * Helper function to encode a string as UTF-16 for usage w/ credential APIs.
 * Returns:
 * Some(val) if the string was successfully converted to UTF-16,
 * or None otherwise.
 */
fn encode_utf16(str: &str) -> Option<Vec<u16>> {
  let mut chars: Vec<u16> = OsStr::new(str).encode_wide().collect();
  if chars.iter().any(|b| *b == 0u16) {
    return None;
  }

  chars.push(0u16);
  Some(chars)
}

pub fn set_password(
  service: &String,
  account: &String,
  password: &mut String,
) -> Result<bool, KeytarError> {
  // Build WinAPI strings and object parameters from arguments
  let target_bytes: Vec<u16> = match encode_utf16(format!("{}/{}", service, account).as_str()) {
    None => {
      return Err(KeytarError::InvalidArg {
        argument: "service/account".to_string(),
        details: "Service/account could not be converted to UTF-16.".to_string(),
      })
    }
    Some(val) => val,
  };
  let username_bytes: Vec<u16> = match encode_utf16(account.as_str()) {
    None => {
      return Err(KeytarError::InvalidArg {
        argument: "username".to_string(),
        details: "Username could not be converted to UTF-16.".to_string(),
      })
    }
    Some(val) => val,
  };

  let cred = CREDENTIALW {
    Flags: 0,
    Type: CRED_TYPE_GENERIC,
    TargetName: target_bytes.as_ptr() as PWSTR,
    Comment: std::ptr::null_mut(),
    LastWritten: FILETIME {
      dwLowDateTime: 0,
      dwHighDateTime: 0,
    },
    Persist: CRED_PERSIST_ENTERPRISE,
    CredentialBlobSize: password.len() as u32,
    CredentialBlob: password.as_ptr() as *mut u8,
    AttributeCount: 0,
    Attributes: std::ptr::null_mut(),
    TargetAlias: std::ptr::null_mut(),
    UserName: username_bytes.as_ptr() as PWSTR,
  };

  // Save credential to user's credential set
  let write_result: i32;
  unsafe {
    write_result = CredWriteW(&cred, 0);
  }

  let error_code: WIN32_ERROR;
  if write_result != TRUE {
    unsafe {
      error_code = GetLastError();
    }
    return Err(KeytarError::from(error_code));
  }

  Ok(true)
}

pub fn get_password(service: &String, account: &String) -> Result<Option<String>, KeytarError> {
  let mut cred: *mut CREDENTIALW = std::ptr::null_mut::<CREDENTIALW>();
  let target_name: Vec<u16> = match encode_utf16(format!("{}/{}", service, account).as_str()) {
    None => {
      return Err(KeytarError::InvalidArg {
        argument: "service/account".to_string(),
        details: "Service/account could not be converted to UTF-16.".to_string(),
      })
    }
    Some(val) => val,
  };

  // Attempt to read credential from user's credential set
  let read_result: i32;
  unsafe {
    read_result = CredReadW(
      target_name.as_ptr() as PCWSTR,
      CRED_TYPE_GENERIC,
      0,
      &mut cred,
    );
  }

  if read_result != TRUE {
    let error_code: WIN32_ERROR;
    unsafe {
      error_code = GetLastError();
    }

    if error_code == ERROR_NOT_FOUND {
      return Ok(None);
    }

    return Err(KeytarError::from(error_code));
  }

  // Build buffer for credential secret and return as UTF-8 string
  unsafe {
    let bytes =
      std::slice::from_raw_parts((*cred).CredentialBlob, (*cred).CredentialBlobSize as usize);

    CredFree(cred as *const c_void);
    return match String::from_utf8(bytes.to_vec()) {
      Ok(str) => Ok(Some(str)),
      Err(err) => Err(KeytarError::Utf8(
        format!("Failed to convert credential to UTF-8: {}", err).to_string(),
      )),
    };
  }
}

pub fn delete_password(service: &String, account: &String) -> Result<bool, KeytarError> {
  let target_name: Vec<u16> = match encode_utf16(format!("{}/{}", service, account).as_str()) {
    None => {
      return Err(KeytarError::InvalidArg {
        argument: "service/account".to_string(),
        details: "Service/account could not be converted to UTF-16.".to_string(),
      })
    }
    Some(val) => val,
  };

  // Attempt to delete credential from user's credential set
  let delete_result: i32;
  unsafe {
    delete_result = CredDeleteW(target_name.as_ptr() as PCWSTR, CRED_TYPE_GENERIC, 0);
  }

  if delete_result != TRUE {
    let error_code: WIN32_ERROR;
    unsafe {
      error_code = GetLastError();
    }

    if error_code == ERROR_NOT_FOUND {
      // If we are trying to delete a credential that doesn't exist,
      // we didn't actually delete the password
      return Ok(false);
    }

    return Err(KeytarError::from(error_code));
  }

  Ok(true)
}

pub fn find_password(service: &String) -> Result<Option<String>, KeytarError> {
  let filter: Vec<u16> = match encode_utf16(format!("{}*", service).as_str()) {
    None => {
      return Err(KeytarError::InvalidArg {
        argument: "service".to_string(),
        details: "Service could not be converted to UTF-16.".to_string(),
      })
    }
    Some(val) => val,
  };

  let mut count: u32 = 0;
  let mut creds: *mut *mut CREDENTIALW = std::ptr::null_mut::<*mut CREDENTIALW>();

  // Attempt to find matching credential from user's credential set
  let find_result: i32;
  unsafe {
    find_result = CredEnumerateW(
      filter.as_ptr() as PCWSTR,
      0u32,
      &mut count,
      &mut creds as *mut *mut *mut CREDENTIALW,
    );
  }

  if find_result != TRUE {
    let error_code: WIN32_ERROR;
    unsafe {
      error_code = GetLastError();
    }
    if error_code == ERROR_NOT_FOUND {
      return Ok(None);
    }

    return Err(KeytarError::from(error_code));
  }

  let cred: *const CREDENTIALW;
  unsafe {
    cred = *creds.offset(0);
    let size = (*cred).CredentialBlobSize as usize;
    let pw = String::from(std::str::from_utf8(std::slice::from_raw_parts(
      (*cred).CredentialBlob,
      size,
    ))?);
    CredFree(creds as *const c_void);

    Ok(Some(pw))
  }
}

pub fn find_credentials(
  service: &String,
  credentials: &mut Vec<(String, String)>,
) -> Result<bool, KeytarError> {
  let filter_bytes: Vec<u16> = match encode_utf16(format!("{}*", service).as_str()) {
    None => {
      return Err(KeytarError::InvalidArg {
        argument: "service".to_string(),
        details: "Service could not be converted to UTF-16.".to_string(),
      })
    }
    Some(val) => val,
  };
  let filter = filter_bytes.as_ptr() as PCWSTR;

  let mut count: u32 = 0;
  let mut creds: *mut *mut CREDENTIALW = std::ptr::null_mut::<*mut CREDENTIALW>();

  // Attempt to fetch user's credential set
  let find_result: i32;
  unsafe {
    find_result = CredEnumerateW(
      filter,
      0u32,
      &mut count,
      &mut creds as *mut *mut *mut CREDENTIALW,
    );
  }

  if find_result != TRUE {
    let error_code: WIN32_ERROR;
    unsafe {
      error_code = GetLastError();
    }
    if error_code == ERROR_NOT_FOUND {
      return Ok(false);
    }

    return Err(KeytarError::from(error_code));
  }

  // Find and build matching credential list from user's credential set
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
      password = String::from(std::str::from_utf8(std::slice::from_raw_parts(
        cred.CredentialBlob,
        cred.CredentialBlobSize as usize,
      ))?);
    }

    let username: String;
    unsafe {
      username = String::from_utf16(std::slice::from_raw_parts(
        cred.UserName,
        uaw_wcslen(cred.UserName),
      ))?;
    }
    credentials.push((username, password));
  }

  unsafe {
    CredFree(creds as *const c_void);
  }

  Ok(true)
}
