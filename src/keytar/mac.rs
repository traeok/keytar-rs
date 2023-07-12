extern crate security_framework;
use super::error::KeytarError;

use security_framework::{
  item::{ItemClass, ItemSearchOptions},
  os::macos::keychain::SecKeychain,
};

const ERR_SEC_ITEM_NOT_FOUND: i32 = -25300;

impl From<security_framework::base::Error> for KeytarError {
  fn from(error: security_framework::base::Error) -> Self {
    KeytarError::Library {
      name: "security_framework".to_string(),
      details: format!("{:?}", error),
    }
  }
}

pub fn set_password(
  service: &String,
  account: &String,
  password: &mut String,
) -> Result<bool, KeytarError> {
  let keychain = SecKeychain::default().unwrap();
  match keychain.set_generic_password(service.as_str(), account.as_str(), password.as_bytes()) {
    Ok(()) => Ok(true),
    Err(err) => Err(KeytarError::from(err)),
  }
}

pub fn get_password(service: &String, account: &String) -> Result<Option<String>, KeytarError> {
  let keychain = SecKeychain::default().unwrap();
  match keychain.find_generic_password(service.as_str(), account.as_str()) {
    Ok((pw, _)) => Ok(Some(String::from_utf8(pw.to_owned())?)),
    Err(err) if err.code() == ERR_SEC_ITEM_NOT_FOUND => Ok(None),
    Err(err) => Err(KeytarError::from(err)),
  }
}

pub fn find_password(service: &String) -> Result<Option<String>, KeytarError> {
  let cred_attrs: Vec<&str> = service.split("/").collect();
  if cred_attrs.len() < 2 {
    return Err(KeytarError::InvalidArg {
      argument: "service".to_string(),
      details: "Invalid format for service string; must be in format 'SERVICE/ACCOUNT'".to_string(),
    });
  }

  let keychain = SecKeychain::default().unwrap();
  match keychain.find_generic_password(cred_attrs[0], cred_attrs[1]) {
    Ok((pw, _)) => {
      let pw_str = String::from_utf8(pw.to_owned())?;
      return Ok(Some(pw_str));
    }
    Err(err) => Err(KeytarError::from(err)),
  }
}

pub fn delete_password(service: &String, account: &String) -> Result<bool, KeytarError> {
  let keychain = SecKeychain::default().unwrap();
  match keychain.find_generic_password(service.as_str(), account.as_str()) {
    Ok((_, item)) => {
      item.delete();
      return Ok(true);
    },
    Err(err) if err.code() == ERR_SEC_ITEM_NOT_FOUND => Ok(false),
    Err(err) => Err(KeytarError::from(err)),
  }
}

pub fn find_credentials(
  service: &String,
  credentials: &mut Vec<(String, String)>,
) -> Result<bool, KeytarError> {
  let search_results = ItemSearchOptions::new()
    .class(ItemClass::generic_password())
    .label(service.as_str())
    .limit(i32::MAX as i64)
    .load_attributes(true)
    .load_data(true)
    .load_refs(true)
    .search()?;

  for result in search_results {
    if let Some(result_map) = result.simplify_dict() {
      credentials.push((
        result_map.get("acct").unwrap().to_string(),
        result_map.get("v_Data").unwrap().to_string(),
      ))
    }
  }
  Ok(!credentials.is_empty())
}
