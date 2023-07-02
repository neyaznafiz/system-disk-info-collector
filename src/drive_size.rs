use std::process::Command;

pub fn drive_size() -> String {
  let output = Command::new("wmic")
  .arg("diskdrive")
  .arg("get")
  .arg("size")
  .output()
  .expect("failed to execute process");

  let size = String::from_utf8(output.stdout.to_vec()).unwrap();
  return size;
}