
mod config;
mod errors;
mod handlers;
mod models;
mod services;
mod utils;

/// Entry Point
///
/// - Start program
/// - parse CLI args
/// - call handelers
/// > commands
/// ```bash
///
///
// ```
fn main() {
    services::system_collector::main();
}



