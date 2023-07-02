mod drive_name;

use std::process::Command;

fn main()  {
   drive_serial_number();
  println!("{}",  drive_name::drive_name());
}

fn drive_serial_number() {
  let output = Command::new("wmic")
  .arg("diskdrive")
  .arg("get")
  .arg("name,")
  .arg("size,")
  .arg("model,")
  .arg("serialnumber")
  .output()
  .expect("failed to execute process");

  let result = String::from_utf8(output.stdout.to_vec()).unwrap();
// io::stdout().write_all(&output.stdout).unwrap();

println!("{}", result);

}