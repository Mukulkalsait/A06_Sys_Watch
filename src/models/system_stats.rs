// Y: Data Models:
// > Contain only MODELS no Data Logics.

/// Data Model: SystemStats
/// - cpu_usage
/// - memory_usage
/// - memory_total
/// - disk_usage
/// - disk_total
/// - process_count
/// - uptime
pub struct SystemStats {
    pub cpu_usage: f32,
    pub memory_usage: u64,
    pub memory_total: u64,
    // pub disk_usage: f64,
    // pub disk_total: f64,
    pub process_count: usize,
    pub uptime: u64,
}
