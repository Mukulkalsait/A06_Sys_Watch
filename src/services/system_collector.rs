// B: Collectin of System Information
// - Read: cpu, memory, disk,
// - Count: processes.

use crate::models::system_stats::SystemStats;
use sysinfo::System;

pub fn collect_system_stats() -> SystemStats {
    let mut sys = System::new();
    sys.refresh_all();
    SystemStats {
        cpu_usage: sys.global_cpu_usage(),
        memory_usage: sys.used_memory(),
        memory_total: sys.total_memory(),
        process_count: sys.processes().len(),
        uptime: System::uptime(),
    }
}

pub fn print_data() {
    let stats = collect_system_stats();
    let divider: u64 = 1024 * 1024;

    println!("------------SystemStats------------");
    println!("CPU usage: {}%", stats.cpu_usage);
    println!("Memory Usage: {} /{} mb", stats.memory_usage / &divider, stats.memory_total / &divider);
    println!("Processes: {}", stats.process_count);
    println!("Uptime: {}sec", stats.uptime);
}

// use std::io::Cursor;
// use sysinfo::{Component, Cpu, Disk, NetworkData, System};
//
// pub fn main() {
//     let mut sys = System::new_all();
//     sys.refresh_all();
//
//     println!("=============| SYSTEM |=============");
//     println!("Total Mem: {}", sys.total_memory());
//     println!("Used Mem: {}", sys.used_memory());
//     println!("Processes: {}", sys.processes().len());
//     println!("CPU cores: {}", sys.cpus().len());
//     println!("GPU Usage: {}", sys.global_cpu_usage());
// }
