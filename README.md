## Drive Information Collector
A package for collect information about disk drive means SSD/HDD etc.

You can collect disk information from any device with just a function call. you will be able to collect the `disk name`, `disk model`, `disk size` and `disk serial number` information using this package.

### **Functions**

- `drive_name()` for collect the name of system disk drive.
- `drive_model()` for collect the model of system disk drive.
- `drive_size()` for collect the total capacity of system disk drive.
- `drive_serial_number()` for collect the serial number of system disk drive.

### **Example**
We are printing here the total capacity information about the disk drive of a system.


```
src/main.rs
-------------------

mod drive_size;

fn main() {
  let size = drive_size::drive_size();
  println!("Disk Drive Size: {}",  size);
}
```
```
--- Output ---

Disk Drive Size: 512105932800 
```

The function `drive_size()` that we called in the `main` function in `main.rs`, we implemented it in the file called `drive_size.rs`, you will find the file on `src/drive_size.rs` .