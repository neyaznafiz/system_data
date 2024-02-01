use std::{process::Command, error, result};
use crate::sanitize_data::{sanitize, GetInfoError};

pub fn name() -> Result<String, GetInfoError>  {
  let output = Command::new("wmic")
  .arg("cpu")
  .arg("get")
  .arg("name")
  .output()
  .expect("failed to execute process");
  
  sanitize(output)
}

pub fn cores() -> Result<String, GetInfoError>  {
  let output = Command::new("wmic")
  .arg("cpu")
  .arg("get")
  .arg("numberofcores")
  .output()
  .expect("failed to execute process");
  
  sanitize(output)
}

pub fn processor_id() -> Result<String, GetInfoError>  {
  let output = Command::new("wmic")
  .arg("cpu")
  .arg("get")
  .arg("processorid")
  .output()
  .expect("failed to execute process");
  
  sanitize(output)
}


#[derive(Debug)]
pub struct CpuInfo {
  pub name: String,
  pub cores: String,
  pub processor_id: String,
}

pub fn all() -> Result<CpuInfo, GetInfoError>  {
  let result = name();
  if let Err(error) = result {
    return Err(error);
  }
  let name = result.unwrap();

  let result = cores();
  if let Err(error) = result {
    return Err(error);
  }
  let cores = result.unwrap();

  let result = processor_id();
  if let Err(error) = result {
    return Err(error);
  }
  let processor_id = result.unwrap();

  let data = CpuInfo {
    name,
    cores,
    processor_id
  };

  Ok(data)
}