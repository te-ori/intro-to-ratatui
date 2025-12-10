mod app;
mod ui;

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend, style::Color, widgets::ListState};
use std::io;

use app::*;

pub struct SomeSettings {
    default_component_border_color: Color,
    active_component_border_color: Color,
    default_menu_item_fg: Color,
    active_menu_item_fg: Color,
}

fn main() -> io::Result<()> {
    // 1. Startup: Enable raw mode and enter alternate screen
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new_with_dummy();
    let mut menu_state = ListState::default();
    // 2. Main Loop
    loop {
        // Draw the UI
        terminal.draw(|f| {
            match menu_state.selected() {
                Some(index) => {
                    app.set_current_note_index(index).unwrap();
                }
                None => {
                    app.reset_current_node_index();
                }
            }

            ui::render(f, &app, &mut menu_state);
        })?;

        // Handle Events
        if let Event::Key(key) = event::read()? {
            match app.mode() {
                AppMode::Normal => {
                    if key.is_press() {
                        match key.code {
                            KeyCode::Char('q') => break,
                            KeyCode::Down => menu_state.select_next(),
                            KeyCode::Up => menu_state.select_previous(),
                            KeyCode::Enter => app.set_to_editing(),
                            _ => {}
                        }
                    }
                }
                AppMode::Editing => {
                    if key.code == KeyCode::Esc {
                        app.set_to_normal();
                        continue;
                    }

                    if key.is_press() {
                        if let Some(note) = app.current_note_mut() {
                            match key.code {
                                KeyCode::Char(c) => note.insert_char_at_current_position(c),
                                KeyCode::Backspace => {
                                    note.remove_char_at_current_position();
                                }
                                KeyCode::Enter => {
                                    note.insert_char_at_current_position('\n');
                                }
                                KeyCode::Left => note.move_cursor_previos(),
                                KeyCode::Right => note.move_cursor_next(),
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
