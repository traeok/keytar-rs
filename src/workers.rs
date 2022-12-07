use napi::{bindgen_prelude::Array, Env, Error, JsBoolean, JsString, Result, Task};
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

pub struct DeletePassword {
  pub service: String,
  pub account: String,
}

pub struct FindCredentials {
  pub service: String,
}

#[napi(object)]
pub struct Credential {
  pub username: String,
  pub password: String,
}

#[napi]
impl Task for GetPassword {
  type Output = String;
  type JsValue = JsString;

  fn compute(&mut self) -> Result<Self::Output> {
    match keytar::get_password(&self.service, &self.account) {
      Ok(pw) => Ok(pw),
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
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
    match keytar::set_password(&self.service, &self.account, &mut self.password) {
      Ok(result) => Ok(result),
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  fn resolve(&mut self, env: Env, output: Self::Output) -> Result<Self::JsValue> {
    env.get_boolean(output)
  }

  fn reject(&mut self, env: Env, err: Error) -> Result<Self::JsValue> {
    env.get_boolean(false)
  }
}

#[napi]
impl Task for DeletePassword {
  type Output = bool;
  type JsValue = JsBoolean;

  fn compute(&mut self) -> Result<Self::Output> {
    match keytar::delete_password(&self.service, &self.account) {
      Ok(result) => Ok(result),
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  fn resolve(&mut self, env: Env, output: Self::Output) -> Result<Self::JsValue> {
    env.get_boolean(output)
  }

  fn reject(&mut self, env: Env, err: Error) -> Result<Self::JsValue> {
    env.get_boolean(false)
  }
}

#[napi]
impl Task for FindCredentials {
  type Output = Vec<(String, String)>;
  type JsValue = Array;

  fn compute(&mut self) -> Result<Self::Output> {
    let mut credentials: Self::Output = Vec::new();
    match keytar::find_credentials(&self.service, &mut credentials) {
      Ok(result) => Ok(credentials),
      Err(err) => Err(napi::Error::from_reason(err.to_string())),
    }
  }

  fn resolve(&mut self, env: Env, output: Self::Output) -> Result<Self::JsValue> {
    let mut arr = env.create_array(0).unwrap();
    for cred in output {
      arr
        .insert(Credential {
          username: cred.0,
          password: cred.1,
        })
        .unwrap();
    }

    Ok(arr)
  }

  fn reject(&mut self, env: Env, err: Error) -> Result<Self::JsValue> {
    Err(err)
  }
}
