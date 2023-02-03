extern crate secret_service;
use std::collections::HashMap;

use secret_service::{EncryptionType, SecretService};

use super::error::KeytarError;

impl From<secret_service::Error> for KeytarError {
  fn from(err: secret_service::Error) -> Self {
    KeytarError::Library {
      name: "secret_service".to_string(),
      details: format!("{:?}", err),
    }
  }
}

pub fn set_password(
  service: &String,
  account: &String,
  password: &mut String,
) -> Result<bool, KeytarError> {
  let ss = SecretService::new(EncryptionType::Dh)?;

  let collection = ss.get_default_collection()?;
  let mut properties = HashMap::new();
  properties.insert("service", service.as_str());
  properties.insert("account", account.as_str());
  match collection.create_item(
    format!("{}/{}", service, account).as_str(),
    properties,
    password.as_bytes(),
    false,
    "text/plain",
  ) {
    Ok(_item) => Ok(true),
    Err(err) => Err(KeytarError::from(err)),
  }
}

pub fn get_password(service: &String, account: &String) -> Result<Option<String>, KeytarError> {
  let ss = SecretService::new(EncryptionType::Dh)?;

  match ss.search_items(vec![("service", service), ("account", account)]) {
    Ok(item) => match item.get(0) {
      Some(it) => {
        let bytes = it.get_secret()?;
        return Ok(Some(String::from_utf8(bytes)?));
      }
      None => Ok(None),
    },
    Err(err) => Err(KeytarError::from(err)),
  }
}

pub fn find_password(service: &String) -> Result<Option<String>, KeytarError> {
  let ss = SecretService::new(EncryptionType::Dh)?;

  let collection = ss.get_default_collection()?;

  let items = collection.get_all_items()?;
  for item in items {
    let label = item.get_label()?;
    if label.contains(service) {
      let bytes = item.get_secret()?;
      let pw = String::from_utf8(bytes)?;
      return Ok(Some(pw));
    }
  }

  Ok(None)
}

pub fn delete_password(service: &String, account: &String) -> Result<bool, KeytarError> {
  let ss = SecretService::new(EncryptionType::Dh)?;

  match ss.search_items(vec![("service", service), ("account", account)]) {
    Ok(item) => match item.get(0) {
      Some(it) => {
        it.delete()?;
        return Ok(true);
      }
      None => Err(KeytarError::NotFound),
    },
    Err(err) => Err(KeytarError::from(err)),
  }
}

pub fn find_credentials(
  service: &String,
  credentials: &mut Vec<(String, String)>,
) -> Result<bool, KeytarError> {
  let ss = SecretService::new(EncryptionType::Dh)?;

  let collection = ss.get_default_collection()?;

  let items = collection.get_all_items()?;
  for item in items {
    let label = item.get_label()?;
    if label.contains(service) {
      let cred: Vec<&str> = label.split("/").collect();
      let bytes = item.get_secret()?;
      let pw = String::from_utf8(bytes)?;
      if cred.is_empty() {
        credentials.push((String::default(), pw));
      } else {
        credentials.push((cred[1].to_string(), pw));
      }
    }
  }

  Ok(!credentials.is_empty())
}
