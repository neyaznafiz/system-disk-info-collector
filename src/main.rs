mod drive;
use drive::{driveName, driveModel, driveSize, driveSerialNumber};

fn main()  {
  println!("Drive Name: {}",  driveName::drive_name());
  println!("Drive Model: {}",  driveModel::drive_model());
  println!("Drive Size: {}",  driveSize::drive_size());
  println!("Drive Serial Number: {}",  driveSerialNumber::drive_serial_number());
}