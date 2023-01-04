use crate::keytar::error::Error;
use std::{io::Write, process::Command};

pub fn set_password(
  service: &String,
  account: &String,
  password: &mut String,
) -> Result<bool, Error> {
  match Command::new("pass")
    .arg("insert")
    .arg("-f")
    .arg(format!("{}/{}", service, account))
    .spawn()
  {
    Ok(mut child) => {
      child
        .stdin
        .as_ref()
        .unwrap()
        .write(password.as_bytes())
        .unwrap();
      child.wait().unwrap();
      Ok(true)
    }
    Err(err) => Err(Error::from(err)),
  }
}

pub fn get_password(service: &String, account: &String) -> Result<String, Error> {
  Ok(
    String::from_utf8(
      Command::new("pass")
        .arg("show")
        .arg(format!("{}/{}", service, account))
        .output()
        .unwrap()
        .stdout,
    )
    .unwrap(),
  )
}

pub fn find_password(service: &String) -> Result<String, Error> {
  Ok(
    String::from_utf8(
      Command::new("pass")
        .arg("find")
        .arg(service)
        .output()
        .unwrap()
        .stdout,
    )
    .unwrap(),
  )
}

pub fn delete_password(service: &String, account: &String) -> Result<bool, Error> {
  println!(
    "{}",
    String::from_utf8(
      Command::new("pass")
        .arg("rm")
        .arg("-f")
        .arg(format!("{}/{}", service, account))
        .output()
        .unwrap()
        .stdout
    )
    .unwrap()
  );
  Ok(false)
}

pub fn find_credentials(
  service: &String,
  credentials: &mut Vec<(String, String)>,
) -> Result<bool, Error> {
  println!(
    "{}",
    String::from_utf8(
      Command::new("pass")
        .arg("find")
        .arg(service)
        .output()
        .unwrap()
        .stdout,
    )
    .unwrap()
  );
  Ok(false)
}
