use crate::keytar::error::Error;
use std::{
  io::Write,
  process::{Command, Stdio},
};

extern crate nc;
use nc::*;

pub fn set_password(
  service: &String,
  account: &String,
  password: &mut String,
) -> Result<bool, Error> {
    
}

pub fn get_password(service: &String, account: &String) -> Result<String, Error> {

}

pub fn find_password(service: &String) -> Result<String, Error> {

}

pub fn delete_password(service: &String, account: &String) -> Result<bool, Error> {

}

pub fn find_credentials(
  service: &String,
  credentials: &mut Vec<(String, String)>,
) -> Result<bool, Error> {

}
