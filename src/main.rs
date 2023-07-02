mod drive_name;
mod drive_model;

use std::process::Command;

fn main()  {
   drive_serial_number();
  println!("{}",  drive_name::drive_name());
  println!("{}",  drive_model::drive_model());
}

fn drive_serial_number() {
  let output = Command::new("wmic")
  .arg("diskdrive")
  .arg("get")
  .arg("size,")
  .arg("model,")
  .arg("serialnumber")
  .output()
  .expect("failed to execute process");

  let result = String::from_utf8(output.stdout.to_vec()).unwrap();
// io::stdout().write_all(&output.stdout).unwrap();

println!("{}", result);

}