use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Layout},
    style::{Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Padding},
};
use std::io;

pub struct App {
    notes: Vec<String>,
    menu_state: ListState,
}

impl App {
    pub fn new() -> App {
        App {
            notes: Vec::new(),
            menu_state: ListState::default(),
        }
    }

    pub fn new_with_dummy() -> App {
        App {
            notes: vec![
                "Note 1".to_string(),
                "Note 2".to_string(),
                "Note 3".to_string(),
                "Note 4".to_string(),
                "Note 5".to_string(),
                "Note 6".to_string(),
                "Note 7".to_string(),
            ],
            menu_state: ListState::default(),
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

    let mut app = App::new_with_dummy();

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

            // # Menu
            // ## Creating `ListItem`'s
            let list_items: Vec<ListItem> = app
                .notes
                .iter()
                .map(|note| ListItem::new(note.as_str()))
                .collect();

            // Creating `List` widget

            let menu_block = Block::default()
                .title(format!("Notes [{}]", app.notes.len()))
                .padding(Padding::new(1, 1, 1, 1))
                .borders(Borders::ALL);
            let list = List::new(list_items).block(menu_block).highlight_style(
                Style::default()
                    .fg(ratatui::style::Color::Green)
                    .add_modifier(Modifier::BOLD),
            );

            f.render_stateful_widget(list, layout[0], &mut app.menu_state);

            // # Editor
            let editor_block = Block::default().title("Editor").borders(Borders::ALL);
            f.render_widget(editor_block, layout[1]);
        })?;

        // Handle Events
        if let Event::Key(key) = event::read()? {
            if key.is_press() {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Down => app.menu_state.select_next(),
                    KeyCode::Up => app.menu_state.select_previous(),
                    _ => {}
                }
            }
        }
    }

    // 3. Shutdown: Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
