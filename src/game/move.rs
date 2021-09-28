use super::piece::Piece;

#[derive(Copy, Clone)]
pub enum Move {
    Left(Piece),
    Right(Piece),
}

impl Move {
    pub fn unwrap(&self) -> Piece {
        match self {
            Move::Left(piece) => *piece,
            Move::Right(piece) => *piece,
        }
    }

    pub fn exposed_value(&self) -> usize {
        match self {
            Move::Left(piece) => piece.left,
            Move::Right(piece) => piece.right,
        }
    }
}