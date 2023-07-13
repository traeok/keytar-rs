extern crate libsecret;
use libsecret::{prelude::RetrievableExtManual, traits::RetrievableExt, SearchFlags};
use std::collections::HashMap;

use super::error::KeytarError;

impl From<glib::error::Error> for KeytarError {
  fn from(err: glib::error::Error) -> Self {
    KeytarError::Library {
      name: "glib".to_owned(),
      details: format!("{:?}", err.message().to_owned()),
    }
  }
}

pub fn set_password(
  service: &String,
  account: &String,
  password: &String,
) -> Result<bool, KeytarError> {
  let attributes = HashMap::from([("service", service.as_str()), ("account", account.as_str())]);

  let collection = libsecret::COLLECTION_DEFAULT;
  match libsecret::password_store_sync(
    None,
    attributes,
    Some(collection),
    format!("{}/{}", service, account).as_str(),
    password.as_str(),
    gio::Cancellable::NONE,
  ) {
    Ok(_) => Ok(true),
    Err(err) => Err(KeytarError::from(err)),
  }
}

pub fn get_password(service: &String, account: &String) -> Result<Option<String>, KeytarError> {
  let attributes = HashMap::from([("service", service.as_str()), ("account", account.as_str())]);

  match libsecret::password_lookup_sync(None, attributes, gio::Cancellable::NONE) {
    Ok(pw) => match pw {
      Some(pass) => Ok(Some(pass.to_string())),
      None => Ok(None),
    },
    Err(err) => Err(KeytarError::from(err)),
  }
}

pub fn find_password(service: &String) -> Result<Option<String>, KeytarError> {
  let attributes = if service.contains("/") && service.len() > 1 {
    // In format "service/account"
    let values: Vec<&str> = service.split("/").collect();
    HashMap::from([("service", values[0]), ("account", values[1])])
  } else {
    HashMap::from([("service", service.as_str())])
  };

  match libsecret::password_lookup_sync(None, attributes, gio::Cancellable::NONE) {
    Ok(pw) => match pw {
      Some(pass) => Ok(Some(pass.to_string())),
      None => Ok(None),
    },
    Err(err) => Err(KeytarError::from(err)),
  }
}

pub fn delete_password(service: &String, account: &String) -> Result<bool, KeytarError> {
  match libsecret::password_clear_sync(
    None,
    HashMap::from([("service", service.as_str()), ("account", account.as_str())]),
    gio::Cancellable::NONE,
  ) {
    Ok(_) => Ok(true),
    Err(err) => match err.kind() {
      Some(glib::KeyFileError::NotFound) => Ok(false),
      _ => Err(KeytarError::from(err)),
    },
  }
}

pub fn find_credentials(
  service: &String,
  credentials: &mut Vec<(String, String)>,
) -> Result<bool, KeytarError> {
  match libsecret::password_search_sync(
    None,
    HashMap::from([("service", service.as_str())]),
    SearchFlags::ALL,
    gio::Cancellable::NONE,
  ) {
    Ok(vec) => {
      let valid_creds: Vec<(String, String)> = vec
        .into_iter()
        .map(|c| {
          let attrs = c.attributes();
          match c.retrieve_secret_sync(gio::Cancellable::NONE) {
            Ok(secret) => Some((attrs.get("account").unwrap().clone(), secret)),
            Err(_) => None,
          }
        })
        .filter(|v| v.is_some())
        .filter_map(|val| {
          let (acc, pass) = val.unwrap();

          let value_obj = pass.unwrap();
          
          let bytes = value_obj.get();
          let pw = String::from_utf8(bytes).unwrap_or("".to_string());
          if pw.is_empty() {
            return None;
          }

          return Some((acc, pw));
        })
        .collect();
      *credentials = valid_creds;

      Ok(true)
    }
    Err(err) => {
      if err.message().contains("No such secret item at path") {
        Ok(false)
      } else {
        Err(KeytarError::Os(err.message().to_owned()))
      }
    },
  }
}
