pub mod system_diskinfo;
pub use crate::system_diskinfo::{driveName, driveModel, driveSize, driveSerialNumber};

fn main()  {

  let name = driveName::drive_name();
  let model = driveModel::drive_model();
  let size = driveSize::drive_size();
  let serial_number = driveSerialNumber::drive_serial_number();

  println!("{}", name);
  println!("{}", model);
  println!("{}", size);
  println!("{}", serial_number);
}