#[derive(Debug)]
pub enum PieceSource {
    Original,
    Add,
}
#[derive(Debug)]
pub struct Piece {
    start: usize,
    length: usize,
    source: PieceSource,
}
#[derive(Debug)]
pub struct PieceTable {
    pub original: String,
    pub add: String,
    pub pieces: Vec<Piece>,
}

impl PieceTable {
    pub fn new(original: String) -> PieceTable {
        let length = original.len();
        PieceTable {
            original,
            add: String::new(),
            pieces: vec![Piece {
                start: 0,
                length,
                source: PieceSource::Original,
            }],
        }
    }

    pub fn insert(&mut self, pos: usize, text: &str) {
        let add_pos = self.add.len();
        self.add += text;

        let piece_idx = self.find_piece(pos);
        let piece = self.pieces.get(piece_idx).unwrap();

        self.pieces.push(Piece {
            source: PieceSource::Add,
            start: add_pos,
            length: text.len(),
        });
    }

    fn find_piece(&self, pos: usize) -> usize {
        let mut offset = 0;
        for (idx, piece) in self.pieces.iter().enumerate() {
            if offset + piece.length > pos {
                return idx;
            }
            offset += piece.length;
        }
        self.pieces.len() - 1
    }

    pub fn get_text(&self) -> String {
        let mut result = String::new();
        println!("{:?}", self.pieces);
        for piece in self.pieces.iter() {
            let source = match piece.source {
                PieceSource::Original => &self.original,
                PieceSource::Add => &self.add,
            };
            result += &source[piece.start..(piece.start + piece.length)];
        }
        result
    }
}
