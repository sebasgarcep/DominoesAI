use crate::piece::Piece;

pub struct Hand {
    pub pieces: Vec<Piece>,
}

impl Hand {
    pub fn new() -> Self {
        Hand {
            pieces: vec![],
        }
    }

    pub fn size(&self) -> usize {
        self.pieces.len()
    }

    pub fn value(&self) -> usize {
        self.pieces.iter().map(|&p| p.value()).sum()
    }
}