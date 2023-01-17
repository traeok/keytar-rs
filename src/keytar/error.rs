use thiserror::Error;

#[derive(Error, Debug)]
pub enum KeytarError {
  #[error("[keytar-rs] An OS error has occurred:\n\n{0}")]
  OSError(String),
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
