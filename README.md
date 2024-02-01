## System Disk Information Collector
A package for collect information about OS, CPU and Disk Drive of a windows operating system.

You can collect disk information from any device with just a function call. you will be able to collect the `OS Information`, `CPU Information` and `Disk Information` information of a windows system with this package.

**Install Package**
```
cargo add system_data
```

**Add Dependencies**
```
system_data = "0.1.0"
```

### **Modules**
- `CpuInfo`
- `DiskInfo`

### **Functions**

**Common Functions**
- `all()` you can use it to get all information.
- `name()` to get the name of CPU/DiskDrive.

**Specific for CpuInfo module**
- `cores()` for collect the cores information of a CPU.
- `processor_id()` for collect the Id of a Processor.

**Specific for DiskInfo module**
- `drive_size()` for collect the total capacity of disk drive.
- `drive_model()` for collect the model of system disk drive.
- `drive_serial_number()` for collect the serial number of system disk drive.

### **Example**
We are printing here the total capacity information about the disk drive of a windows system.

*cargo.toml*
```
[dependencies]
system_data = "0.1.0"
```

*main.rs*
```
use system_data;

fn main() {
  let cpu_name = system_data::CpuInfo::name();
  println!("CPU Name: {}",  cpu_name);
}
```

Open your terminal with the correct path of your project and run `cargo run` command to get the output.


### Best Regards.
