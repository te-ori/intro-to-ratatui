mod app;

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

use app::*;

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
                .titles()
                .map(|title| ListItem::new(format!("{}", title)))
                .collect();

            // Creating `List` widget
            let menu_border_color = if app.is_in_normal_mode() {
                CURRENT_SETTINGS.active_component_border_color
            } else {
                CURRENT_SETTINGS.default_component_border_color
            };
            let menu_block = Block::default()
                .title(format!("Notes [{}]", app.notes_count()))
                .padding(Padding::new(1, 1, 1, 1))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(menu_border_color));

            let list = List::new(list_items).block(menu_block).highlight_style(
                Style::default()
                    .fg(CURRENT_SETTINGS.active_menu_item_fg)
                    .add_modifier(Modifier::BOLD),
            );

            f.render_stateful_widget(list, layout[0], &mut menu_state);

            // # Editor
            let editor_border_color = if app.is_in_edit_mode() {
                CURRENT_SETTINGS.active_component_border_color
            } else {
                CURRENT_SETTINGS.default_component_border_color
            };
            let editor_block = Block::default()
                .title("Editor")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(editor_border_color));
            // let editor_content = app
            //     .menu_state
            //     .selected()
            //     .map(|index| app.notes[index].note.content.clone())
            //     .unwrap_or_else(|| "Nothing selected".to_string());
            let editor_content = match app.current_note() {
                Some(note) => note.content(),
                None => "",
            };

            // implement visual aid for cursor position

            let paragraph = Paragraph::new(editor_content).block(editor_block);

            if app.is_in_edit_mode()
                && let Some(note) = app.current_note()
            {
                let s = &note.content()[0..note.cursor_position()];
                let last_line_break_index = s.rfind('\n').map(|i| i + 1).unwrap_or(0);
                let x = note.cursor_position() - last_line_break_index;
                let y = s.chars().filter(|&c| c == '\n').count();

                f.set_cursor_position(Position::new(
                    layout[1].x + 1 + x as u16,
                    layout[1].y + 1 + y as u16,
                ));
            }

            f.render_widget(paragraph, layout[1]);

            // Footer
            let footer_content = if let Some(note) = app.current_note() {
                format!(
                    "Note Count: {} | Char Count: {} | Position: {} | Date: {}",
                    app.notes_count(),
                    note.content().len(),
                    note.cursor_position(),
                    note.date()
                )
            } else {
                format!("Note Count: {}", app.notes_count())
            };
            let footer =
                Paragraph::new(footer_content).block(Block::default().borders(Borders::ALL));
            f.render_widget(footer, main_layout[1]);
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
