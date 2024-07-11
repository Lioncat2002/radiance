use std::fs;

pub struct Document {
    pub data: String,
    pub lines: usize,
}

impl Document {
    pub fn new(filepath: &str) -> Document {
        let data = fs::read_to_string(filepath).expect("File not found! [document::Document::new]");
        let line_data: Vec<&str> = data.split('\n').collect();
        let lines = line_data.len();
        Document { data, lines }
    }

    pub fn empty() -> Document {
        let data = String::new();
        Document { data, lines: 0 }
    }
}
