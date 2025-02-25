## System Disk Information Collector
A package for collect information about disk drive means SSD/HDD etc of a windows operating system.

You can collect disk information from any device with just a function call. you will be able to collect the `disk name`, `disk model`, `disk size` and `disk serial number` information of a windows system with this package.

**Install Package**
```
cargo add system_diskinfo
```

**Add Dependencies**
```
system_diskinfo = "0.2.2"
```

### **Modules**
- `driveName`
- `driveSize`
- `driveModel`
- `driveSerialNumber`

### **Functions**
- `drive_name()` for collect the name of system disk drive.
- `drive_size()` for collect the total capacity of disk drive.
- `drive_model()` for collect the model of system disk drive.
- `drive_serial_number()` for collect the serial number of system disk drive.

### **Guideline**
First of all add this package to your `dependencies` in `cargo.toml` file, then open a file where you want to use and add the package in the top of the file like `use system_diskinfo`, after that, to get the correct output use it like this: 
- First write the package name. ( `system_diskinfo` )
- Add double clone. ( `::` )
- Write module name using camel case. ( `driveName` )
- Add double clone. ( `::` )
- Call the function using snake case. ( `drive_name` )

### **Example**
We are printing here the total capacity information about the disk drive of a windows system.

*cargo.toml*
```
[dependencies]
system_diskinfo = "0.2.2"
```

*main.rs*
```
use system_diskinfo;

fn main() {
  let size = system_diskinfo::driveSize::drive_size();
  println!("Disk Drive Size: {}",  size);
}
```

Open your terminal with the correct path of your project and run `cargo run` command
```
--- Output ---

Disk Drive Size: 512105932800 
```

### Best Regards.
