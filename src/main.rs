pub mod disk_info;
pub use disk_info as DiskInfo;

pub mod cpu_info;
pub use cpu_info as CpuInfo;

mod sanitize_data;

fn main()  {
  // let disk_name = DiskInfo::name();
  // let disk_model = DiskInfo::model();
  // let disk_size = DiskInfo::size();
  // let disk_serial_number = DiskInfo::serial_number();

  // println!("{:?}", disk_name);
  // println!("{:?}", disk_model);
  // println!("{:?}", disk_size);
  // println!("{:?}", disk_serial_number);

  let cpu_name = CpuInfo::name();
  // let cpu_cores = CpuInfo::cores();
  // let cpu_id = CpuInfo::processor_id();

  println!("{:?}", cpu_name);
  // println!("{:?}", cpu_cores);
  // println!("{:?}", cpu_id);
}