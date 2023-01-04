use crate::keytar::error::Error;
use std::{
  io::Write,
  process::{Command, Stdio},
};

pub fn set_password(
  service: &String,
  account: &String,
  password: &mut String,
) -> Result<bool, Error> {
  match Command::new("pass")
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .arg("insert")
    .arg("-e")
    .arg("-f")
    .arg(format!("{}/{}", service, account))
    .spawn()
  {
    Ok(mut child) => {
      child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(password.as_bytes())?;

      child.wait().unwrap();
      Ok(true)
    }
    Err(err) => Err(Error::from(err)),
  }
}

pub fn get_password(service: &String, account: &String) -> Result<String, Error> {
  let mut bytes = Command::new("pass")
    .arg("show")
    .arg(format!("{}/{}", service, account))
    .output()
    .unwrap()
    .stdout;

  // pop newline appended by pass
  if bytes[bytes.len() - 1] == 0xA {
    bytes.pop();
  }
  Ok(String::from_utf8(bytes).unwrap())
}

pub fn find_password(service: &String) -> Result<String, Error> {
  if service.contains('/') {
    let contents: Vec<&str> = service.split('/').collect();
    return get_password(&contents[0].to_string(), &contents[1].to_string());
  }
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
  Ok(
    String::from_utf8(
      Command::new("pass")
        .arg("rm")
        .arg("-f")
        .arg(format!("{}/{}", service, account))
        .output()
        .unwrap()
        .stdout,
    )
    .unwrap()
    .contains("removed '"),
  )
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
