use std::fs;

use super::core::piece_table::PieceTable;

pub struct Document {
    pub data: PieceTable,
    pub lines: usize,
}

impl Document {
    pub fn new(filepath: &str) -> Document {
        let data = fs::read_to_string(filepath).expect("File not found! [document::Document::new]");
        let line_data: Vec<&str> = data.split('\n').collect();
        let lines = line_data.len();
        Document {
            data: PieceTable::new(data),
            lines,
        }
    }

    pub fn empty() -> Document {
        let original = "".to_string();
        Document {
            data: PieceTable::new(original),
            lines: 0,
        }
    }
}
