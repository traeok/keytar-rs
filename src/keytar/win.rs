use std::ffi::c_void;
use std::result::Result;
use windows::{core::*, Win32::Foundation::*, Win32::Security::Credentials::*};

pub fn set_password(service: String, account: String, mut password: String) -> bool {
    let mut cred: CREDENTIALW = CREDENTIALW::default();
    cred.Type = CRED_TYPE_GENERIC;
    let mut target_bytes: Vec<u16> = format!("{}/{}", service, account).encode_utf16().collect();
    target_bytes.push(0);
    cred.TargetName = PWSTR::from_raw(target_bytes.as_mut_ptr());
    let mut username_bytes: Vec<u16> = account.encode_utf16().collect();
    cred.UserName = PWSTR::from_raw(username_bytes.as_mut_ptr());
    cred.CredentialBlobSize = password.len() as u32;
    cred.CredentialBlob = password.as_mut_ptr();
    cred.Persist = CRED_PERSIST_ENTERPRISE;
    unsafe { bool::from(CredWriteW(&cred, 0)) }
}

pub fn get_password(service: String, account: String, password: &mut String) -> bool {
    let mut cred: *mut *mut CREDENTIALW = std::ptr::null_mut::<*mut CREDENTIALW>();
    let mut target_name: Vec<u16> = format!("{}/{}", service, account).encode_utf16().collect();

    let read_result: bool;
    unsafe {
        read_result = bool::from(CredReadW(
            PCWSTR::from_raw(target_name.as_mut_ptr()),
            CRED_TYPE_GENERIC.0,
            0,
            cred,
        ));
    }

    if !read_result {
        let code: WIN32_ERROR;
        unsafe {
            code = GetLastError();
        }
        if code == ERROR_NOT_FOUND {
            return true;
        }

        return false;
    }

    unsafe {
        let size = (*(*cred)).CredentialBlobSize as usize;
        if password.capacity() < size {
            password.reserve_exact(size - password.capacity());
        }

        std::ptr::copy(password.as_mut_ptr(), (*(*cred)).CredentialBlob, size);
    }

    unsafe {
        CredFree(*cred as *const c_void);
    }

    true
}

pub fn delete_password(service: String, account: String) -> bool {
    let mut target_name: Vec<u16> = format!("{}/{}", service, account).encode_utf16().collect();

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
            return true;
        }

        return false;
    }

    return true;
}

pub fn find_password(service: String, password: &mut String) -> bool {
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
            return true;
        }

        return false;
    }

    let cred: *const CREDENTIALW;
    unsafe {
        cred = *creds.offset(0);
        let size = (*cred).CredentialBlobSize as usize;
        if password.capacity() < size {
            password.reserve_exact(size - password.capacity());
        }
        std::ptr::copy(password.as_mut_ptr(), (*cred).CredentialBlob, size);
    }

    unsafe {
        CredFree(creds as *const c_void);
    }

    true
}

pub fn find_credentials(
    service: String,
    credentials: &mut Vec<(String, String)>,
) -> Result<bool, WIN32_ERROR> {
    let filter = PCWSTR::from_raw(
        format!("{}*", service)
            .encode_utf16()
            .collect::<Vec<u16>>()
            .as_mut_ptr(),
    );

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

        return Err(code);
    }

    for i in 0..count {
        let cred: &CREDENTIALW;
        unsafe {
            cred = &**creds.offset(i as isize);
        }

        if cred.UserName.is_null() || cred.CredentialBlobSize == 0 {
            continue;
        }

        let mut password: String = String::new();
        unsafe {
            password.reserve(cred.CredentialBlobSize as usize);
            std::ptr::copy(
                password.as_mut_ptr(),
                cred.CredentialBlob,
                cred.CredentialBlobSize as usize,
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
