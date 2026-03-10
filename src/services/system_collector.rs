// B: Collectin of System Information
// - Read: cpu, memory, disk,
// - Count: processes.

use sysinfo::{Component, Cpu, Disk, NetworkData, System};

pub fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();

    pub fn abdc() {
        loop {
            println!();
        }
    }

    println!("=============| SYSTEM |=============");
    println!("Total Mem: {}", sys.total_memory());
    println!("Used Mem: {}", sys.used_memory());
    println!("Processes: {}", sys.processes().len());
    println!("CPU cores: {}", sys.cpus().len());
}
