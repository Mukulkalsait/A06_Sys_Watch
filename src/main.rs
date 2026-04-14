mod config;
mod errors;
mod handlers;

// struct SysStat
mod models;

// collector_main
mod services;
use services::system_collector::collect_system_stats;

mod utils;

/// Entry Point
///
/// - Start program
/// - parse CLI args
/// - call handelers
/// > commands
/// ```bash
///
/// ```
fn main() {
    let stats = collect_system_stats();
    let divider: u64 = 1024 * 1024;

    println!("------------SystemStats------------");
    println!("CPU usage: {}%", stats.cpu_usage);
    println!("Memory Usage: {} /{} mb", stats.memory_usage / &divider, stats.memory_total / &divider);
    println!("Processes: {}", stats.process_count);
    println!("Uptime: {}sec", stats.uptime);
}
