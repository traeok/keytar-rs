extern crate secret_service;
use crate::keytar::error::Error;
use secret_service::{EncryptionType, SecretService};
use std::collections::HashMap;

impl From<secret_service::Error> for Error {
  fn from(error: secret_service::Error) -> Self {
    Error {
      code: None,
      details: Some(error.to_string()),
    }
  }
}

pub fn set_password(
  service: &String,
  account: &String,
  password: &mut String,
) -> Result<bool, Error> {
  let ss = SecretService::new(EncryptionType::Dh).unwrap();

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
    Ok(item) => Ok(true),
    Err(err) => Err(Error::from(err)),
  }
}

pub fn get_password(service: &String, account: &String) -> Result<String, Error> {
  let ss = SecretService::new(EncryptionType::Dh)?;

  match ss.search_items(vec![("service", service), ("account", account)]) {
    Ok(item) => match item.get(0) {
      Some(it) => {
        let bytes = it.get_secret().unwrap();
        return Ok(String::from_utf8(bytes).unwrap());
      }
      None => Err(Error::from_details(
        "No items found with the specified attributes",
      )),
    },
    Err(err) => Err(Error::from(err)),
  }
}

// TODO: replace function stubs
pub fn find_password(service: &String) -> Result<String, Error> {
  Ok(String::default())
}

pub fn delete_password(service: &String, account: &String) -> Result<bool, Error> {
  Ok(false)
}

pub fn find_credentials(
  service: &String,
  credentials: &mut Vec<(String, String)>,
) -> Result<bool, Error> {
  Ok(false)
}
