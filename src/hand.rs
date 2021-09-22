use crate::piece::Piece;

pub struct Hand {
    pub pieces: Vec<Piece>,
}

impl Hand {
    pub fn empty() -> Self {
        Hand {
            pieces: vec![],
        }
    }

    pub fn new(pieces: Vec<Piece>) -> Self {
        Hand {
            pieces,
        }
    }

    pub fn contains(&self, piece: &Piece) -> bool {
        self.pieces.contains(piece)
    }

    pub fn size(&self) -> usize {
        self.pieces.len()
    }

    pub fn value(&self) -> usize {
        self.pieces.iter().map(|&p| p.value()).sum()
    }
}