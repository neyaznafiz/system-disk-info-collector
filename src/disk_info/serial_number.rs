use std::process::Command;
use crate::disk_info::DriveError;
use crate::sanitize_data::sanitize;

pub fn drive_serial_number() -> Result<String, DriveError> {
  let output = Command::new("wmic")
  .arg("diskdrive")
  .arg("get")
  .arg("serialnumber")
  .output()
  .expect("failed to execute process");

  sanitize(output)
}