pub struct Cursor {
    pub line: u64,
    pub col: u64,
}

impl Cursor {
    pub fn new(line: u64, col: u64) -> Cursor {
        Cursor { line, col }
    }

    pub fn move_up(&mut self) {
        if self.line == 0 {
            println!("Cursor at top line [cursor::Cursor::move_up]");
            return;
        }
        self.line -= 10;
        self.col = 0;
    }

    pub fn move_down(&mut self) {
        self.line += 10;
        self.col = 0;
    }

    pub fn move_left(&mut self) {
        if self.col == 0 {
            println!("Cursor at left most position of the line {}", self.line);
            self.move_up();
            return;
        }
        self.col -= 10;
    }
    //TODO: think of a way to handle max char length in a line while move right
    pub fn move_right(&mut self) {
        self.col += 10;
    }
}
