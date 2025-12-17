mod app;
mod collaboration;
mod network;
mod ui;

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend, style::Color, widgets::ListState};
use std::{io, process::Command, thread};

use app::*;

pub struct SomeSettings {
    default_component_border_color: Color,
    active_component_border_color: Color,
    default_menu_item_fg: Color,
    active_menu_item_fg: Color,
}

#[tokio::main]
async fn main() -> io::Result<()> {
    // 1. Startup: Enable raw mode and enter alternate screen
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new_with_dummy();
    let mut menu_state = ListState::default();

    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<collaboration::Command>();
    let txx = tx.clone();
    tokio::spawn(async move {
        network::server::start(txx).await;
    });

    let (ktx, mut krx) = tokio::sync::mpsc::unbounded_channel::<Event>();
    let kxx = ktx.clone();
    thread::spawn(move || {
        loop {
            if let Ok(event) = event::read() {
                if kxx.send(event).is_err() {
                    break;
                }
            }
        }
    });

    // 2. Main Loop
    loop {
        terminal.draw(|f| {
            match menu_state.selected() {
                Some(index) => {
                    _ = app.set_current_note_index(index);
                }
                None => {
                    app.reset_current_node_index();
                }
            }

            ui::render(f, &app, &mut menu_state);
        })?;

        tokio::select! {
            command = rx.recv() => {
                match command {
                    Some(cmd) => {
                        // println!("message arrived");

                        match cmd {
                            collaboration::Command::SetCursorPosition(set_cursor_position) => {
                                app.set_collaborator_position(
                                    set_cursor_position.message_index,
                                    set_cursor_position.pos,
                                );
                            }
                        }
                    }
                    None => {}
                }
            }
            event = krx.recv() => {
                if let Some(Event::Key(key)) = event {
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
        }
    }

    // 3. Shutdown: Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
