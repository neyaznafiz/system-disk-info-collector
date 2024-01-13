use std::{process::Command, error};
use crate::disk_info::DriveError;
use crate::sanitize_data::sanitize;

pub fn cpu_name() -> Result<String,DriveError>  {
  let output = Command::new("wmic")
  .arg("cpu")
  .arg("get")
  .arg("name")
  .output()
  .expect("failed to execute process");

  let result = String::from_utf8(output.stdout.to_vec());
  
  sanitize(output)
}