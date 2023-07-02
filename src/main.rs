mod drive_name;
mod drive_model;
mod drive_size;
mod drive_serial_number;

fn main()  {
  println!("{}-{}-{}-{}",  drive_name::drive_name(), drive_model::drive_model(), drive_size::drive_size(), drive_serial_number::drive_serial_number());
}