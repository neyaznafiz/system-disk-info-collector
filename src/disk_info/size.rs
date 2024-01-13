use std::process::Command;
use crate::disk_info::DriveError;
use crate::sanitize_data::sanitize;

pub fn drive_size() -> Result<String, DriveError> {
  let output = Command::new("wmic")
  .arg("diskdrive")
  .arg("get")
  .arg("size")
  .output()
  .expect("failed to execute process");

  sanitize(output)
}