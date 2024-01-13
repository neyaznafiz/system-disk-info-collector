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

  sanitize(output)
}