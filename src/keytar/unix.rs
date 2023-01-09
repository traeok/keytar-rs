use crate::keytar::error::Error;
use crate::providers::keyctl::{types::Keyring, SpecialId};

pub fn set_password(
  service: &String,
  account: &String,
  password: &mut String,
) -> Result<bool, Error> {
  let kring = Keyring::from_special_id(SpecialId::User, true)?;
  let key = kring.add_key(format!("{}/{}", service, account).as_str(), password)?;
  Ok(key.id > 0)
}

pub fn get_password(service: &str, account: &str) -> Result<String, Error> {
  let kring = Keyring::from_special_id(SpecialId::User, true)?;
  let key = kring.search(format!("{}/{}", service, account).as_str())?;
  Ok(key.read_as_utf8()?)
}

pub fn find_password(service: &String) -> Result<String, Error> {
  let kring = Keyring::from_special_id(SpecialId::User, true)?;
  let key = kring.find_key(service)?;
  Ok(key.read_as_utf8()?)
}

pub fn delete_password(service: &String, account: &String) -> Result<bool, Error> {
  let kring = Keyring::from_special_id(SpecialId::User, true)?;
  let key = kring.search(format!("{}/{}", service, account).as_str())?;
  key.invalidate()?;
  Ok(true)
}

pub fn find_credentials(
  service: &String,
  credentials: &mut Vec<(String, String)>,
) -> Result<bool, Error> {
  let kring = Keyring::from_special_id(SpecialId::User, true)?;
  let kc = kring.keys()?;
  for k in kc.into_iter() {
    let metadata = k.describe()?;
    let desc = metadata.split(";").collect::<Vec<&str>>().pop().unwrap();
    if !desc.contains(service) {
      continue;
    }

    credentials.push((desc.replace("\0", "").to_string(), k.read_as_utf8()?));
  }
  Ok(!credentials.is_empty())
}
