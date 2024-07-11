use raylib::prelude::*;

use super::cursor::Cursor;

pub struct InputHandler<'a>{
    cursor: &'a mut Cursor
}

impl<'a> InputHandler<'a>{
    pub fn new(cursor:&'a mut Cursor)->InputHandler<'a>{
        InputHandler{
            cursor
        }
    }
    pub fn process_text_entered(&mut self,rl:&mut RaylibHandle,data:&mut String){
        while let Some(key) = rl.get_char_pressed() {
            data.push(key);
            self.cursor.move_right();
        }
    }
    pub fn process_key_pressed(&mut self,rl:&mut RaylibHandle,data:&mut String){
        if let Some(key) = rl.get_key_pressed() {
            match key {
                KeyboardKey::KEY_ENTER => {
                    data.push('\n');
                    self.cursor.move_down();
                }
                KeyboardKey::KEY_BACKSPACE => {
                    data.pop();
                    self.cursor.move_left()
                }
                _ => (),
            }
        }
    }
}