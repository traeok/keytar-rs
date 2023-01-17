use std::{str::Utf8Error, string::FromUtf16Error, string::FromUtf8Error};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum KeytarError {
  #[error("[keytar-rs] {service:?} library returned an error:\n\n{details:?}")]
  Library { service: String, details: String },

  #[error("[keytar-rs] No items were found that match the given parameters.")]
  NotFound,

  #[error("[keytar-rs] An OS error has occurred:\n\n{0}")]
  Os(String),

  #[error("[keytar-rs] A UTF-8 error has occurred:\n\n{0}")]
  Utf8(String),

  #[error("[keytar-rs] A UTF-16 error has occurred:\n\n{0}")]
  Utf16(String),
}

// TODO: remove and use enum above instead
pub struct Error {
  pub code: Option<i32>,
  pub details: Option<String>,
}

impl Error {
  // not actually dead: only used on certain platforms
  #[allow(dead_code)]
  pub fn from_details(details: &str) -> Self {
    Error {
      code: None,
      details: Some(details.to_string()),
    }
  }
}

impl ToString for Error {
  fn to_string(&self) -> String {
    match self.code {
      Some(code) => match &self.details {
        Some(detail) => format!("[ERR] keytar-rs - code: {}, details: {}", code, detail),
        None => format!("[ERR] keytar-rs - code: {}", code),
      },
      None => match &self.details {
        Some(detail) => format!("[ERR] keytar-rs - details: {}", detail),
        None => format!("[ERR] keytar-rs error - no further info provided."),
      },
    }
  }
}

impl From<FromUtf8Error> for KeytarError {
  fn from(error: FromUtf8Error) -> Self {
    KeytarError::Utf8(format!("{:?}", error))
  }
}

impl From<FromUtf16Error> for KeytarError {
  fn from(error: FromUtf16Error) -> Self {
    KeytarError::Utf16(format!("{:?}", error))
  }
}

impl From<Utf8Error> for KeytarError {
  fn from(error: Utf8Error) -> Self {
    KeytarError::Utf8(format!("{:?}", error))
  }
}
