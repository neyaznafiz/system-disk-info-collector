use std::process::Command;

pub fn drive_serial_number() -> String {
  let output = Command::new("wmic")
  .arg("diskdrive")
  .arg("get")
  .arg("serialnumber")
  .output()
  .expect("failed to execute process");

  let serial_number = String::from_utf8(output.stdout.to_vec()).unwrap();
  return serial_number;
}