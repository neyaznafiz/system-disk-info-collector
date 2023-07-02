use std::process::Command;

pub fn drive_name() -> String {
  let output = Command::new("wmic")
  .arg("diskdrive")
  .arg("get")
  .arg("name")
  .output()
  .expect("failed to execute process");

  let name = String::from_utf8(output.stdout.to_vec()).unwrap();
  return name;
}