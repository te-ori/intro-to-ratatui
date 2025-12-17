pub struct SetCursorPosition {
    pub message_index: usize,
    pub pos: usize,
}

pub enum Command {
    SetCursorPosition(SetCursorPosition),
}
