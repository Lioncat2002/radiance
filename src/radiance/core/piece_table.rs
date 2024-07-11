pub enum PieceSource {
    Original,
    Add,
}

pub struct Piece {
    start: usize,
    length: usize,
    source: PieceSource,
}

pub struct PieceTable {
    original: String,
    add: String,
    pieces: Vec<Piece>,
}

impl PieceTable {
    pub fn new(original: String) -> PieceTable {
        PieceTable {
            original,
            add: String::new(),
            pieces: Vec::new(),
        }
    }

    pub fn insert(&mut self, pos: usize, text: &str) {
        let add_pos = self.add.len();
        self.add += text;

        let piece_idx = self.find_piece(pos);
        let piece = self.pieces.get(piece_idx).unwrap();

        if piece.start + piece.length > pos {
            //incase of insert in between
        } else if self.pieces.len() <= 1 {
            //edge case when less than 2 element present in piece table
        } else {
            self.pieces.insert(
                piece_idx,
                Piece {
                    source: PieceSource::Add,
                    start: add_pos,
                    length: text.len(),
                },
            );
        }
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
}
