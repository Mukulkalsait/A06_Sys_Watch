use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};

use crate::app::App;

pub fn draw(f: &mut Frame, app: &App) {
    // widgit 1 data
    let size = f.area();
    let block = Block::default().title("📊 SysWatch").borders(Borders::ALL);

    // widgit 2 data
    let text = format!("CPU cores: {}\nMemory: {}/{} Mb", app.cpu_usage.len(), app.memory_used / 1024, app.memory_total / 1024);
    let paragraph = Paragraph::new(text);

    // both widgets
    f.render_widget(block, size);
    f.render_widget(paragraph, size);
}
