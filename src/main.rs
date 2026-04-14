use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::{self, CrosstermBackend},
    macros::ratatui_core::terminal,
};
use std::{
    io,
    time::{Duration, Instant},
};

mod app;
use app::App;

mod config;
mod errors;
mod handlers;
mod ui;

// struct SysStat
mod models;

// collector_main
mod services;
use services::system_collector::print_data;

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
fn main() -> Result<(), io::Error> {
    // test one
    // print_data();
    // --------------------------------------------------------
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();

    let tick_rate = Duration::from_millis(500);
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| ui::draw(f, &app))?;

        let timeout = tick_rate.checked_sub(last_tick.elapsed()).unwrap_or_else(|| Duration::from_secs(0));
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.update();
            last_tick = Instant::now();
        }
    }
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}
