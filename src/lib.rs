use napi::bindgen_prelude::AsyncTask;
use napi_derive::napi;
use workers::{DeletePassword, FindCredentials, GetPassword, SetPassword};

mod keytar;
mod workers;

#[napi]
fn delete_password(service: String, account: String) -> AsyncTask<DeletePassword> {
  AsyncTask::new(DeletePassword { service, account })
}

#[napi]
fn find_credentials(service: String) -> AsyncTask<FindCredentials> {
  AsyncTask::new(FindCredentials { service })
}

#[napi]
fn get_password(service: String, account: String) -> AsyncTask<GetPassword> {
  AsyncTask::new(GetPassword { service, account })
}

#[napi]
fn set_password(service: String, account: String, password: String) -> AsyncTask<SetPassword> {
  AsyncTask::new(SetPassword {
    service,
    account,
    password,
  })
}
