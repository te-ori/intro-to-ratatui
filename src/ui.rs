use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Position},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Padding, Paragraph},
};

use crate::{SomeSettings, app::App};

pub static CURRENT_SETTINGS: SomeSettings = SomeSettings {
    default_component_border_color: Color::White,
    active_component_border_color: Color::Yellow,
    default_menu_item_fg: Color::White,
    active_menu_item_fg: Color::Green,
};

pub fn render(f: &mut Frame, app: &App, menu_state: &mut ListState) {
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

    let list = List::new(list_items)
        .block(menu_block)
        .style(Style::default().fg(CURRENT_SETTINGS.default_menu_item_fg))
        .highlight_style(
            Style::default()
                .fg(CURRENT_SETTINGS.active_menu_item_fg)
                .add_modifier(Modifier::BOLD),
        );

    f.render_stateful_widget(list, layout[0], menu_state);

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
    let editor_content = match app.current_note() {
        Some(note) => note.content(),
        None => "",
    };

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
    let footer = Paragraph::new(footer_content).block(Block::default().borders(Borders::ALL));
    f.render_widget(footer, main_layout[1]);
}
