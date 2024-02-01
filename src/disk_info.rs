use std::{process::Command, error};
use crate::sanitize_data::{sanitize, GetInfoError};

pub fn name() -> Result<String, GetInfoError> {
  let output = Command::new("wmic")
  .arg("diskdrive")
  .arg("get")
  .arg("name")
  .output()
  .expect("failed to execute process");

  sanitize(output)
}

pub fn model() -> Result<String,GetInfoError>  {
  let output = Command::new("wmic")
  .arg("diskdrive")
  .arg("get")
  .arg("model")
  .output()
  .expect("failed to execute process");

  sanitize(output)
}

pub fn serial_number() -> Result<String, GetInfoError> {
  let output = Command::new("wmic")
  .arg("diskdrive")
  .arg("get")
  .arg("serialnumber")
  .output()
  .expect("failed to execute process");

  sanitize(output)
}

pub fn size() -> Result<String, GetInfoError> {
  let output = Command::new("wmic")
  .arg("diskdrive")
  .arg("get")
  .arg("size")
  .output()
  .expect("failed to execute process");

  sanitize(output)
}

#[derive(Debug)]
pub struct DiskInfo {
  pub name: String,
  pub model: String,
  pub serial_number: String,
  pub size: String
}

pub fn all() -> Result<DiskInfo, GetInfoError>  {
  let result = name();
  if let Err(error) = result {
    return Err(error);
  }
  let name = result.unwrap();

  let result = model();
  if let Err(error) = result {
    return Err(error);
  }
  let model = result.unwrap();

  let result = serial_number();
  if let Err(error) = result {
    return Err(error);
  }
  let serial_number = result.unwrap();

  let result = size();
  if let Err(error) = result {
    return Err(error);
  }
  let size = result.unwrap();

  let data = DiskInfo {
    name,
    model,
    serial_number,
    size
  };

  Ok(data)
}