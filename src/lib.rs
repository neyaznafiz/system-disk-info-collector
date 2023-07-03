mod drive;
pub use drive::{driveName, driveModel, driveSize, driveSerialNumber};

/// A package for collect information about disk drive means SSD/HDD etc.
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
/// We are printing here the total capacity information about the disk drive of a system.
///  
/// ```
/// src/main.rs
/// --------------
/// 
/// mod drive;
/// use drive::{driveSize};
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
  let name = driveName::drive_name();
  let model = driveModel::drive_model();
  let size = driveSize::drive_size();
  let serial_number = driveSerialNumber::drive_serial_number();

  println!("{}", name);
  println!("{}", model);
  println!("{}", size);
  println!("{}", serial_number);
}