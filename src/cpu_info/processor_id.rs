use std::{process::Command, error};
use crate::disk_info::DriveError;
use crate::sanitize_data::sanitize;

pub fn id() -> Result<String,DriveError>  {
  let output = Command::new("wmic")
  .arg("cpu")
  .arg("get")
  .arg("processorid")
  .output()
  .expect("failed to execute process");

  let result = String::from_utf8(output.stdout.to_vec());

  if let Err(error) = result {
    return Err(DriveError::FromUtf8Error);
  }

  let data = result.unwrap();
  println!("{:?}", data);
  
  sanitize(output)
}