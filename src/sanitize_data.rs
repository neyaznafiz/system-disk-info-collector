use std::{process::Output, error};
use crate::disk_info::DriveError;

pub fn sanitize(output: Output) -> Result<String, DriveError> {
  let result = String::from_utf8(output.stdout.to_vec());

  if let Err(error) = result {
    return Err(DriveError::FromUtf8Error);
  }

  let data = result.unwrap();
  
  let arr:Vec<&str> = data.split("\n").collect();
  if arr.len() < 2 {
    return Err(DriveError::DriveNotFound);
  }

  let name = arr[1].to_string();
  let arr:Vec<&str> = name.split("\r").collect();
  if arr.len() < 1 {
    return Err(DriveError::DriveNotFound);
  }

  return Ok(arr[0].trim().to_string());
}