use std::process::Command;

pub fn drive_model() -> String {
  let output = Command::new("wmic")
  .arg("diskdrive")
  .arg("get")
  .arg("model")
  .output()
  .expect("failed to execute process");

  let result = String::from_utf8(output.stdout.to_vec()).unwrap();
  let model: Vec<&str> = result.split("\n").collect();

  return model[1].to_string();
}