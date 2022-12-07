extern crate security_framework;
use crate::keytar::error::Error;
use security_framework::passwords::{
  delete_generic_password, get_generic_password, set_generic_password,
};

impl From<security_framework::base::Error> for Error {
  fn from(error: security_framework::base::Error) -> Self {
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

// TODO: replace function stubs
pub fn find_password(service: &String) -> Result<String, Error> {
  Ok(String::default())
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
  Ok(false)
}
