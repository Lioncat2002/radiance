use raylib::prelude::*;

use super::{core::piece_table::PieceTable, cursor::Cursor, document::Document};

pub struct InputHandler<'a> {
    cursor: &'a mut Cursor,
}

impl<'a> InputHandler<'a> {
    pub fn new(cursor: &'a mut Cursor) -> InputHandler<'a> {
        InputHandler { cursor }
    }
    pub fn process_text_entered(&mut self, rl: &mut RaylibHandle, document: &mut Document) {
        while let Some(key) = rl.get_char_pressed() {
            document
                .data
                .insert(document.data.original.len(), &key.to_string());
            self.cursor.move_right();
            document.characters += 1;
        }
    }
    pub fn process_key_pressed(&mut self, rl: &mut RaylibHandle, document: &mut Document) {
        if let Some(key) = rl.get_key_pressed() {
            match key {
                KeyboardKey::KEY_ENTER => {
                    document.data.insert(document.data.original.len(), "\n");
                    document.lines += 1;
                    self.cursor.move_down();
                }
                KeyboardKey::KEY_BACKSPACE => {
                    //data.pop();
                    if document.characters > 0 {
                        document.data.pop(document.characters - 1, 1);
                        self.cursor.move_left();
                        document.characters -= 1;
                    } else {
                        println!("No characters in buffer");
                    }
                }
                _ => (),
            }
        }
    }
}
