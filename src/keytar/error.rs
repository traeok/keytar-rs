pub struct Error {
  code: Option<u32>,
  details: Option<String>,
}

impl Error {
  pub fn from_win(code: u32) -> Self {
    Error {
      code: Some(code),
      details: None,
    }
  }

  pub fn from_unix(err: &dyn std::error::Error) -> Self {
    Error {
      code: None,
      details: Some(err.to_string()),
    }
  }

  // TODO: update as needed
  pub fn from_mac(err: &dyn std::error::Error) -> Self {
    Error {
      code: None,
      details: Some(err.to_string()),
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
