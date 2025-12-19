#[derive(PartialEq, Clone, Copy)]
pub enum AppMode {
    Normal,
    Editing,
}

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
    collaborator_position: usize,
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

    pub fn content(&self) -> &str {
        &self.note.content
    }

    pub fn title(&self) -> &str {
        &self.note.title
    }

    pub fn date(&self) -> &str {
        &self.note.date
    }

    pub fn cursor_position(&self) -> usize {
        self.cursor_position
    }

    pub fn collaborator_position(&self) -> usize {
        self.collaborator_position
    }
}

pub struct App {
    notes: Vec<EditorNote>,
    current_node_index: Option<usize>,
    current_mode: AppMode,
}

impl App {
    pub fn new() -> App {
        App {
            notes: Vec::new(),
            current_node_index: None,
            current_mode: AppMode::Normal,
        }
    }

    pub fn new_with_dummy() -> App {
        App {
            notes: vec![
                EditorNote {
                    cursor_position: 0,
                    note: Note::new("Note 1".to_string(), "Content of Note 1".to_string()),
                    collaborator_position: 0,
                },
                EditorNote {
                    cursor_position: 0,
                    note: Note::new("Note 2".to_string(), "Content of Note 2".to_string()),
                    collaborator_position: 0,
                },
                EditorNote {
                    cursor_position: 0,
                    note: Note::new("Note 3".to_string(), "Content of Note 3".to_string()),
                    collaborator_position: 0,
                },
                EditorNote {
                    cursor_position: 0,
                    note: Note::new("Note 4".to_string(), "Content of Note 4".to_string()),
                    collaborator_position: 0,
                },
                EditorNote {
                    cursor_position: 0,
                    note: Note::new("Note 5".to_string(), "Content of Note 5".to_string()),
                    collaborator_position: 0,
                },
                EditorNote {
                    cursor_position: 0,
                    note: Note::new("Note 6".to_string(), "Content of Note 6".to_string()),
                    collaborator_position: 0,
                },
            ],
            current_node_index: None,
            current_mode: AppMode::Normal,
        }
    }

    pub fn set_current_note_index(&mut self, index: usize) -> Result<(), String> {
        if index >= self.notes.len() {
            return Err("index can not be greater than item count".to_string());
        }

        self.current_node_index = Some(index);
        Ok(())
    }

    pub fn reset_current_node_index(&mut self) {
        self.current_node_index = None;
    }

    pub fn current_note_mut(&mut self) -> Option<&mut EditorNote> {
        if self.current_node_index.is_none() {
            return None;
        }

        Some(&mut self.notes[self.current_node_index.unwrap()])
    }

    pub fn current_note(&self) -> Option<&EditorNote> {
        if self.current_node_index.is_none() {
            return None;
        }

        Some(&self.notes[self.current_node_index.unwrap()])
    }
    pub fn current_note_id(&self) -> Option<usize> {
        if self.current_node_index.is_none() {
            return None;
        }

        Some(self.current_node_index.unwrap())
    }

    pub fn titles(&self) -> impl Iterator<Item = &String> {
        self.notes.iter().map(|n| &n.note.title)
    }

    pub fn notes_count(&self) -> usize {
        self.notes.len()
    }

    pub fn mode(&self) -> AppMode {
        self.current_mode
    }

    pub fn set_to_editing(&mut self) {
        self.current_mode = AppMode::Editing
    }

    pub fn set_to_normal(&mut self) {
        self.current_mode = AppMode::Normal
    }

    pub fn is_in_normal_mode(&self) -> bool {
        self.current_mode == AppMode::Normal
    }

    pub fn is_in_edit_mode(&self) -> bool {
        self.current_mode == AppMode::Editing
    }

    pub fn set_collaborator_position(&mut self, note_index: usize, pos: usize) {
        self.notes[note_index].collaborator_position = pos;
    }
}
