use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Layout},
    widgets::{Block, Borders},
};
use std::io;

pub struct App {
    notes: Vec<String>,
}

impl App {
    pub fn new() -> App {
        App { notes: Vec::new() }
    }

    pub fn new_with_dummy() -> App {
        App {
            notes: vec![
                "Note 1".to_string(),
                "Note 2".to_string(),
                "note 3".to_string(),
            ],
        }
    }
}

fn main() -> io::Result<()> {
    // 1. Startup: Enable raw mode and enter alternate screen
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = App::new_with_dummy();
    // 2. Main Loop
    loop {
        // Draw the UI
        terminal.draw(|f| {
            let size = f.area();
            let layout = Layout::new(
                ratatui::layout::Direction::Horizontal,
                vec![Constraint::Percentage(20), Constraint::Percentage(80)],
            )
            .split(size);

            let menu_block = Block::default().title(format!("Notes [{}]", app.notes.len())).borders(Borders::ALL);
            f.render_widget(menu_block, layout[0]);

            let editor_block = Block::default().title("Editor").borders(Borders::ALL);
            f.render_widget(editor_block, layout[1]);
        })?;

        // Handle Events
        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('q') {
                break;
            }
        }
    }

    // 3. Shutdown: Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
