use windows::{core::*, Win32::Foundation::*, Win32::Security::Credentials::*};

fn set_password(service: String, account: String, password: String) -> bool {
    let cred: CREDENTIALW;
    cred.Type = CRED_TYPE_GENERIC;
    cred.TargetName = service + "/" + account;
    cred.UserName = account;
    cred.CredentialBlobSize = password.len();
    cred.CredentialBlob = password.as_bytes();
    cred.Persist = CRED_PERSIST_ENTERPRISE;

    unsafe { CredWriteW(cred, 0) }
}

fn get_password(service: String, account: String, password: &mut String) -> bool {
    let cred: CREDENTIALW;
    let target_name = service + "/" + account;

    let read_result: bool;
    unsafe {
        read_result = CredReadW(target_name.as_bytes(), CRED_TYPE_GENERIC, 0, &mut cred);
    }

    if (!read_result) {
        let code = GetLastError();
        if (code == ERROR_NOT_FOUND) {
            return true;
        }

        false
    }

    unsafe {
        CredFree(&cred as *const c_void);
    }

    true
}

fn delete_password(service: String, account: String) {
    let target_name = service + "/" + account;

    let delete_result: bool;
    unsafe {
        delete_result = CredDelete(target_name.as_bytes(), CRED_TYPE_GENERIC, 0);
    }

    if (!delete_result) {
        let code = GetLastError();
        if (code == ERROR_NOT_FOUND) {
            return true;
        }

        false
    }

    true
}

fn find_password(service: String, password: &mut String) {
    let filter = service + "*";

    let count: u32;
    let creds: *mut *mut CREDENTIALW;

    let result: bool;
    unsafe {
        result = CredEnumerate(
            filter.as_bytes(),
            0,
            &mut count,
            &mut creds as *mut *mut *mut CREDENTIALW,
        );
    }

    if (!result) {
        let code = GetLastError();
        if (code == ERROR_NOT_FOUND) {
            return true;
        }

        false
    }

    let cred: CREDENTIALW = *creds.offset(0);
    std::ptr::copy(
        password.as_mut_ptr(),
        cred.CredentialBlob,
        cred.CredentialBlobSize,
    );

    unsafe {
        CredFree(creds);
    }

    true
}

fn find_credentials(service: String, credentials: &mut Vec<(String, String)>) {
    let filter = service + "*";

    let count: u32;
    let creds: *mut *mut CREDENTIALW;

    let result: bool;
    unsafe {
        result = CredEnumerate(
            filter,
            0,
            &mut count,
            &mut creds as *mut *mut *mut CREDENTIALW,
        );
    }

    if (!result) {
        let code = GetLastError();
        if (code == ERROR_NOT_FOUND) {
            return true;
        }

        false
    }

    for i in range(0..count) {
        let cred: &CREDENTIALW = *creds.offset(i);

        if (cred.UserName == NULL || cred.CredentialBlobSize == NULL) {
            continue;
        }

        let password: String;
        std::ptr::copy(
            password.as_mut_ptr(),
            cred.CredentialBlob,
            cred.CredentialBlobSize,
        );

        credentials.push((cred.UserName.to_string(), password));
    }

    unsafe {
        CredFree(creds);
    }

    true
}
