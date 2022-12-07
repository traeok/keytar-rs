extern crate security_framework;
use crate::keytar::error::Error;
use security_framework::item::ItemSearchOptions;
use security_framework::{
  os::macos::passwords::find_generic_password,
  passwords::{delete_generic_password, get_generic_password, set_generic_password},
};

impl From<security_framework::base::Error> for Error {
  fn from(error: security_framework::base::Error) -> Self {
    Error {
      code: None,
      details: Some(error.to_string()),
    }
  }
}

impl From<std::string::FromUtf8Error> for Error {
  fn from(error: std::string::FromUtf8Error) -> Self {
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
  match set_generic_password(service.as_str(), account.as_str(), password.as_bytes()) {
    Ok(()) => Ok(true),
    Err(err) => Err(Error::from(err)),
  }
}

pub fn get_password(service: &String, account: &String) -> Result<String, Error> {
  match get_generic_password(service.as_str(), account.as_str()) {
    Ok(bytes) => Ok(String::from_utf8(bytes).unwrap()),
    Err(err) => Err(Error::from(err)),
  }
}

pub fn find_password(service: &String) -> Result<String, Error> {
  let cred_attrs: Vec<&str> = service.split("/").collect();
  if cred_attrs.len() < 2 {
    return Err(Error::from_details(
      "Improper service string syntax for find_password",
    ));
  }

  match find_generic_password(None, cred_attrs[0], cred_attrs[1]) {
    Ok((pw, item)) => {
      let pw_str = String::from_utf8(pw.to_owned())?;
      return Ok(pw_str);
    }
    Err(err) => Err(err),
  }
}

pub fn delete_password(service: &String, account: &String) -> Result<bool, Error> {
  match delete_generic_password(service.as_str(), account.as_str()) {
    Ok(_) => Ok(true),
    Err(err) => Err(Error::from(err)),
  }
}

pub fn find_credentials(
  service: &String,
  credentials: &mut Vec<(String, String)>,
) -> Result<bool, Error> {
  let search_builder = ItemSearchOptions::new()
    .label(service.as_str())
    .load_attributes(true)
    .load_refs(true);

  let results = search_builder.search()?;

  // TODO: get username and populate credentials
  Ok(false)
}
