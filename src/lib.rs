use napi::bindgen_prelude::AsyncTask;
use napi_derive::napi;
use workers::{GetPassword, SetPassword};

mod keytar;
mod workers;

#[napi]
fn set_password(service: String, account: String, password: String) -> AsyncTask<SetPassword> {
  AsyncTask::new(SetPassword {
    service,
    account,
    password,
  })
}

#[napi]
fn get_password(service: String, account: String) -> AsyncTask<GetPassword> {
  AsyncTask::new(GetPassword { service, account })
}
