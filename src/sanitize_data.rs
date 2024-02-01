use std::{process::Output, error};

#[derive(Debug)]
pub enum GetInfoError {
  FromUtf8Error,
  DriveNotFound
}

pub fn sanitize(output: Output) -> Result<String, GetInfoError> {
  let result = String::from_utf8(output.stdout.to_vec());

  if let Err(error) = result {
    return Err(GetInfoError::FromUtf8Error);
  }

  let data = result.unwrap();
  
  let arr:Vec<&str> = data.split("\n").collect();
  if arr.len() < 2 {
    return Err(GetInfoError::DriveNotFound);
  }

  let name = arr[1].to_string();
  let arr:Vec<&str> = name.split("\r").collect();
  if arr.len() < 1 {
    return Err(GetInfoError::DriveNotFound);
  }

  return Ok(arr[0].trim().to_string());
}