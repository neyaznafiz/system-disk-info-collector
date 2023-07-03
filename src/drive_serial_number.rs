use std::process::Command;

pub fn drive_serial_number() -> String {
  let output = Command::new("wmic")
  .arg("diskdrive")
  .arg("get")
  .arg("serialnumber")
  .output()
  .expect("failed to execute process");

  let result = String::from_utf8(output.stdout.to_vec()).unwrap();
  let serial_number:Vec<&str> = result.split("\n").collect();

  return serial_number[1].to_string();
}