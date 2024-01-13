use std::{process::Command, error};
use crate::disk_info::DriveError;
use crate::sanitize_data::sanitize;

pub fn drive_model() -> Result<String,DriveError>  {
  let output = Command::new("wmic")
  .arg("diskdrive")
  .arg("get")
  .arg("model")
  .output()
  .expect("failed to execute process");

  sanitize(output)
}