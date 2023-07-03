mod drive_name;
mod drive_model;
mod drive_size;
mod drive_serial_number;

fn main()  {
  println!("Drive Name: {}",  drive_name::drive_name());
  println!("Drive Model: {}",  drive_model::drive_model());
  println!("Drive Size: {}",  drive_size::drive_size());
  println!("Drive Serial Number: {}",  drive_serial_number::drive_serial_number());
}