use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Position},
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

pub struct Note {
    title: String,
    content: String,
    date: String,
}

impl Note {
    pub fn new(title: String, content: String) -> Self {
        Note {
            title,
            content,
            date: "01.01.2025 18:30".to_string(),
        }
    }
}

pub struct EditorNote {
    cursor_position: usize,
    note: Note,
}

impl EditorNote {
    pub fn move_cursor_next(&mut self) {
        if self.cursor_position < self.note.content.len() {
            self.cursor_position += 1;
        }
    }
    pub fn move_cursor_previos(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
        }
    }

    pub fn insert_char_at_current_position(&mut self, c: char) {
        self.note.content.insert(self.cursor_position, c);
        self.move_cursor_next();
    }

    pub fn remove_char_at_current_position(&mut self) {
        if self.cursor_position == 0 {
            return;
        }

        _ = self.note.content.remove(self.cursor_position - 1);
        self.move_cursor_previos();
    }
}

pub struct App {
    notes: Vec<EditorNote>,
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
                EditorNote {
                    cursor_position: 0,
                    note: Note::new("Note 1".to_string(), "Content of Note 1".to_string()),
                },
                EditorNote {
                    cursor_position: 0,
                    note: Note::new("Note 2".to_string(), "Content of Note 2".to_string()),
                },
                EditorNote {
                    cursor_position: 0,
                    note: Note::new("Note 3".to_string(), "Content of Note 3".to_string()),
                },
                EditorNote {
                    cursor_position: 0,
                    note: Note::new("Note 4".to_string(), "Content of Note 4".to_string()),
                },
                EditorNote {
                    cursor_position: 0,
                    note: Note::new("Note 5".to_string(), "Content of Note 5".to_string()),
                },
                EditorNote {
                    cursor_position: 0,
                    note: Note::new("Note 6".to_string(), "Content of Note 6".to_string()),
                },
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

            let main_layout = Layout::new(
                Direction::Vertical,
                vec![Constraint::Min(0), Constraint::Length(3)],
            )
            .split(size);

            let layout = Layout::new(
                ratatui::layout::Direction::Horizontal,
                vec![Constraint::Percentage(20), Constraint::Percentage(80)],
            )
            .split(main_layout[0]);

            // # Menu
            // ## Creating `ListItem`'s
            let list_items: Vec<ListItem> = app
                .notes
                .iter()
                .map(|note| {
                    ListItem::new(format!(
                        "{} - [{}]",
                        note.note.title.as_str(),
                        note.cursor_position
                    ))
                })
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
                .map(|index| app.notes[index].note.content.clone())
                .unwrap_or_else(|| "Nothing selected".to_string());

            // implement visual aid for cursor position

            let paragraph = Paragraph::new(editor_content).block(editor_block);

            if app.current_mode == AppMode::Editing
                && let Some(index) = app.menu_state.selected()
            {
                let s = &app.notes[index].note.content[0..app.notes[index].cursor_position];
                let last_line_break_index = s.rfind('\n').map(|i| i + 1).unwrap_or(0);
                let x = app.notes[index].cursor_position - last_line_break_index;
                let y = s.chars().filter(|&c| c == '\n').count();

                f.set_cursor_position(Position::new(
                    layout[1].x + 1 + x as u16,
                    layout[1].y + 1 + y as u16,
                ));
            }

            f.render_widget(paragraph, layout[1]);

            // Footer
            let footer_content = if let Some(index) = app.menu_state.selected() {
                format!(
                    "Note Count: {} | Char Count: {} | Position: {} | Date: {}",
                    app.notes.len(),
                    app.notes[index].note.content.len(),
                    app.notes[index].cursor_position,
                    app.notes[index].note.date
                )
            } else {
                format!("Note Count: {}", app.notes.len())
            };
            let footer =
                Paragraph::new(footer_content).block(Block::default().borders(Borders::ALL));
            f.render_widget(footer, main_layout[1]);
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
                    if key.code == KeyCode::Esc {
                        app.current_mode = AppMode::Normal;
                        continue;
                    }

                    if key.is_press() {
                        if let Some(index) = app.menu_state.selected() {
                            let current_note = &mut app.notes[index];
                            match key.code {
                                KeyCode::Char(c) => current_note.insert_char_at_current_position(c),
                                KeyCode::Backspace => {
                                    current_note.remove_char_at_current_position();
                                }
                                KeyCode::Enter => {
                                    current_note.insert_char_at_current_position('\n');
                                }
                                KeyCode::Left => current_note.move_cursor_previos(),
                                KeyCode::Right => current_note.move_cursor_next(),
                                _ => {}
                            }
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
