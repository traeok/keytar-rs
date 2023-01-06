use crate::keytar::error::Error;
use crate::providers::keyctl::{types::Keyring, SpecialId};

pub fn set_password(
  service: &String,
  account: &String,
  password: &mut String,
) -> Result<bool, Error> {
  let kring = Keyring::from_special_id(SpecialId::User, true)?;

  kring.add_key(format!("{}/{}", service, account).as_str(), password)?;

  Ok(true)
}

pub fn get_password(service: &str, account: &str) -> Result<String, Error> {
  let kring = Keyring::from_special_id(SpecialId::User, true)?;
  let key = kring.search(format!("{}/{}", service, account).as_str())?;
  let secret = key.read()?;

  Ok(String::from_utf8(secret).unwrap())
}

pub fn find_password(service: &String) -> Result<String, Error> {
  let kring = Keyring::from_special_id(SpecialId::User, true)?;
  let key = kring.search(service)?;
  let secret = key.read()?;

  Ok(String::from_utf8(secret).unwrap())
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

    credentials.push((
      desc.replace("\0", "").to_string(),
      String::from_utf8(k.read()?).unwrap(),
    ));
  }
  Ok(!credentials.is_empty())
}
