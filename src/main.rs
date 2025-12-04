use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Padding, Paragraph},
};
use std::io;

#[derive(PartialEq)]
pub enum AppMode {
    Normal,
    Editing,
}

pub struct SomeSettings {
    default_component_border_color: Color,
    active_component_border_color: Color,
    default_menu_item_fg: Color,
    active_menu_item_fg: Color,
}

pub static CURRENT_SETTINGS: SomeSettings = SomeSettings {
    default_component_border_color: Color::White,
    active_component_border_color: Color::Yellow,
    default_menu_item_fg: Color::White,
    active_menu_item_fg: Color::Green,
};

pub struct App {
    notes: Vec<String>,
    menu_state: ListState,
    current_mode: AppMode,
}

impl App {
    pub fn new() -> App {
        App {
            notes: Vec::new(),
            menu_state: ListState::default(),
            current_mode: AppMode::Normal,
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
            current_mode: AppMode::Normal,
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
            let menu_border_color = if app.current_mode == AppMode::Normal {
                CURRENT_SETTINGS.active_component_border_color
            } else {
                CURRENT_SETTINGS.default_component_border_color
            };
            let menu_block = Block::default()
                .title(format!("Notes [{}]", app.notes.len()))
                .padding(Padding::new(1, 1, 1, 1))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(menu_border_color));

            let list = List::new(list_items).block(menu_block).highlight_style(
                Style::default()
                    .fg(CURRENT_SETTINGS.active_menu_item_fg)
                    .add_modifier(Modifier::BOLD),
            );

            f.render_stateful_widget(list, layout[0], &mut app.menu_state);

            // # Editor
            let editor_border_color = if app.current_mode == AppMode::Editing {
                CURRENT_SETTINGS.active_component_border_color
            } else {
                CURRENT_SETTINGS.default_component_border_color
            };
            let editor_block = Block::default()
                .title("Editor")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(editor_border_color));
            let editor_content = app
                .menu_state
                .selected()
                .map(|index| app.notes[index].clone())
                .unwrap_or_else(|| "Nothing selected".to_string());

            let paragraph = Paragraph::new(editor_content).block(editor_block);
            f.render_widget(paragraph, layout[1]);
        })?;

        // Handle Events
        if let Event::Key(key) = event::read()? {
            match app.current_mode {
                AppMode::Normal => {
                    if key.is_press() {
                        match key.code {
                            KeyCode::Char('q') => break,
                            KeyCode::Down => app.menu_state.select_next(),
                            KeyCode::Up => app.menu_state.select_previous(),
                            KeyCode::Enter => app.current_mode = AppMode::Editing,
                            _ => {}
                        }
                    }
                }
                AppMode::Editing => {
                    if key.is_press() {
                        match key.code {
                            KeyCode::Esc => app.current_mode = AppMode::Normal,
                            KeyCode::Char(c) => {
                                if let Some(index) = app.menu_state.selected() {
                                    app.notes[index].push(c)
                                }
                            }
                            KeyCode::Backspace => {
                                if let Some(index) = app.menu_state.selected() {
                                    _ = app.notes[index].pop()
                                }
                            }
                            _ => {}
                        }
                    }
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
