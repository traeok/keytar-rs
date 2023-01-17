use std::{str::Utf8Error, string::FromUtf16Error, string::FromUtf8Error};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum KeytarError {
  #[error("[keytar-rs] Invalid parameter provided for '{argument:?}'. Details:\n\n{details:?}")]
  InvalidArg { argument: String, details: String },

  #[error("[keytar-rs] {name:?} library returned an error:\n\n{details:?}")]
  Library { name: String, details: String },

  #[error("[keytar-rs] No items were found that match the given parameters.")]
  NotFound,

  #[error("[keytar-rs] An OS error has occurred:\n\n{0}")]
  Os(String),

  #[error("[keytar-rs] A UTF-8 error has occurred:\n\n{0}")]
  Utf8(String),

  #[error("[keytar-rs] A UTF-16 error has occurred:\n\n{0}")]
  Utf16(String),
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
