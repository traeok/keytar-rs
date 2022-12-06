use napi::{Env, Error, JsBoolean, JsString, Result, Task};
use napi_derive::napi;

use crate::keytar;

// TODO: error handling

pub struct SetPassword {
  pub service: String,
  pub account: String,
  pub password: String,
}

pub struct GetPassword {
  pub service: String,
  pub account: String,
}

#[napi]
impl Task for GetPassword {
  type Output = String;
  type JsValue = JsString;

  fn compute(&mut self) -> Result<Self::Output> {
    let result = keytar::get_password(&self.service, &self.account);
    if result.is_ok() {
      return Ok(result.unwrap());
    }

    return Err(napi::Error::from_reason(format!(
      "keytar-rs: Failed to get password: {}",
      result.unwrap_err().0
    )));
  }

  fn resolve(&mut self, env: Env, output: Self::Output) -> Result<Self::JsValue> {
    env.create_string(output.as_str())
  }

  fn reject(&mut self, env: Env, err: Error) -> Result<Self::JsValue> {
    env.create_string(&err.to_string())
  }
}

#[napi]
impl Task for SetPassword {
  type Output = bool;
  type JsValue = JsBoolean;

  fn compute(&mut self) -> Result<Self::Output> {
    if keytar::set_password(&self.service, &self.account, &mut self.password) {
      return Ok(true);
    }

    Err(napi::Error::from_reason(
      "keytar-rs: unable to set password",
    ))
  }

  fn resolve(&mut self, env: Env, output: Self::Output) -> Result<Self::JsValue> {
    env.get_boolean(output)
  }

  fn reject(&mut self, env: Env, err: Error) -> Result<Self::JsValue> {
    env.get_boolean(false)
  }
}
