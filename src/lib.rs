pub mod disk_info;
pub mod cpu_info;
mod sanitize_data;

/// A package for collect information of a system.
///  
/// You can collect disk information from any device with just a function call. you will be able to collect the `disk name`, `disk model`, `disk size` and `disk serial number` information with this package.
/// 
/// ### Functions
/// `drive_name()` for collect the name of system disk drive. </br>
/// `drive_model()` for collect the model of system disk drive. </br>
/// `drive_size()` for collect the total capacity of system disk drive. </br>
/// `drive_serial_number()` for collect the serial number of system disk drive. </br>
///  
/// ### Example
/// We are printing here the total capacity information about the disk drive of a windows system.
///  
/// ```
/// src/main.rs
/// --------------
/// 
/// mod disk_info;
/// use disk_info::{driveSize};
/// 
/// fn main() {
///   let size = driveSize::drive_size();
///   println!("Disk Size: {}",  size);
/// }
/// ```
/// ```
/// --- Output ---
/// 
/// **Disk Size 512105932800**
/// ``` 
/// 
/// The function `drive_size()` that we called in the main function in main.rs, we implemented it in the file called `drive_size.rs`, you will find the file on `src/drive/drive_size.rs`.

pub fn read_doc()  {
  let disk_name = disk_info::Name::drive_name();
  let disk_model = disk_info::Model::drive_model();
  let disk_size = disk_info::Size::drive_size();
  let disk_serial_number = disk_info::SerialNumber::drive_serial_number();

  println!("{:?}", disk_name);
  println!("{:?}", disk_model);
  println!("{:?}", disk_size);
  println!("{:?}", disk_serial_number);

  let cpu_name = cpu_info::Name::cpu_name();
  let cpu_cores = cpu_info::NumberOfCores::cpu_cores();
  let cpu_id = cpu_info::ProcessorID::id();

  println!("{:?}", cpu_name);
  println!("{:?}", cpu_cores);
  println!("{:?}", cpu_id);
}