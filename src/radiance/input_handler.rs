use raylib::prelude::*;

use super::{core::piece_table::PieceTable, cursor::Cursor};

pub struct InputHandler<'a> {
    cursor: &'a mut Cursor,
}

impl<'a> InputHandler<'a> {
    pub fn new(cursor: &'a mut Cursor) -> InputHandler<'a> {
        InputHandler { cursor }
    }
    pub fn process_text_entered(&mut self, rl: &mut RaylibHandle, data: &mut PieceTable) {
        while let Some(key) = rl.get_char_pressed() {
            data.insert(data.original.len(), &key.to_string());
            self.cursor.move_right();
        }
    }
    pub fn process_key_pressed(&mut self, rl: &mut RaylibHandle, data: &mut PieceTable) {
        if let Some(key) = rl.get_key_pressed() {
            match key {
                KeyboardKey::KEY_ENTER => {
                    data.insert(data.original.len(), "\n");
                    self.cursor.move_down();
                }
                KeyboardKey::KEY_BACKSPACE => {
                    //data.pop();
                    self.cursor.move_left()
                }
                _ => (),
            }
        }
    }
}
