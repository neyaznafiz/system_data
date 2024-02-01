pub mod disk_info;
pub use disk_info as DiskInfo;

pub mod cpu_info;
pub use cpu_info as CpuInfo;

mod sanitize_data;

/// A package for collect information about disk drive means SSD/HDD etc of a windows operating system.
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
  let disk_name = DiskInfo::name();
  let disk_model = DiskInfo::model();
  let disk_size = DiskInfo::size();
  let disk_serial_number = DiskInfo::serial_number();

  println!("{:?}", disk_name);
  println!("{:?}", disk_model);
  println!("{:?}", disk_size);
  println!("{:?}", disk_serial_number);

  let cpu_name = CpuInfo::name();
  let cpu_cores = CpuInfo::cores();
  let cpu_id = CpuInfo::processor_id();

  println!("{:?}", cpu_name);
  println!("{:?}", cpu_cores);
  println!("{:?}", cpu_id);
}