pub mod name;
pub use name as Name;

pub mod model;
pub use model as Model;

pub mod size;
pub use size as Size;

pub mod serial_number;
pub use serial_number as SerialNumber;

#[derive(Debug)]
pub enum DriveError {
  FromUtf8Error,
  DriveNotFound
}