pub mod cpu_info;
pub use cpu_info as CpuInfo;

pub mod disk_info;
pub use disk_info as DiskInfo;

mod sanitize_data;

/// A package for collect information about OS, CPU and Disk Drive of a windows operating system.

/// You can collect disk information from any device with just a function call. you will be able to collect the `OS Information`, `CPU Information` and `Disk Information` information of a windows system with this package.
/// 

/// ### Functions
/// `cores()` for collect the cores information of a CPU.
///  
/// ### Example
/// We are printing here the total capacity information about the disk drive of a windows system.
///  
/// ```
/// src/main.rs
/// --------------
/// 
/// pub mod cpu_info;
/// pub use cpu_info as CpuInfo;
/// 
/// fn main() {
///   let cores = CpuInfo::cores();
///   println!("CPU Cores: {}",  cores);
/// }
/// ```

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