extern crate secret_service;
use crate::keytar::error::Error;
use secret_service::{EncryptionType, SecretService};

// TODO
fn set_password(service: String, account: String, password: String) -> Result<bool, Error> {
  let ss = SecretService::new(EncryptionType::Dh);

  let collection = ss.get_default_collection()?;
  collection.create_item(
    format!("{}/{}", service, account),
    password.as_bytes(),
    false,
    "text/plain",
  )?;

  Ok(true)
}
