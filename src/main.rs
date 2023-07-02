mod drive_name;
mod drive_model;
mod drive_size;

use std::process::Command;

fn main()  {
   drive_serial_number();
  println!("{}",  drive_name::drive_name());
  println!("{}",  drive_model::drive_model());
  println!("{}",  drive_size::drive_size());
}

fn drive_serial_number() {
  let output = Command::new("wmic")
  .arg("diskdrive")
  .arg("get")
  .arg("size,")
  .arg("serialnumber")
  .output()
  .expect("failed to execute process");

  let result = String::from_utf8(output.stdout.to_vec()).unwrap();
// io::stdout().write_all(&output.stdout).unwrap();

println!("{}", result);

}