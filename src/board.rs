use crate::hand::Hand;
use crate::piece::Piece;

#[derive(Copy, Clone)]
pub enum Move {
    Left(Piece),
    Right(Piece),
}

pub struct Board {
    pieces: Vec<Piece>,
}

impl Board {
    pub fn new() -> Self {
        Board {
            pieces: vec![],
        }
    }

    pub fn valid_moves(&self, hand: &Hand) -> Vec<Move> {
        let mut valid_moves = vec![];


        for piece in hand.pieces.iter() {
            if let Some(player_move) = self.validate_move(Move::Left(*piece)) {
                valid_moves.push(player_move);
            }
            if let Some(player_move) = self.validate_move(Move::Right(*piece)) {
                valid_moves.push(player_move);
            }
        }

        return valid_moves;
    }

    pub fn validate_move(&self, player_move: Move) -> Option<Move> {
        if self.pieces.len() == 0 {
            return Some(player_move)
        }

        match player_move {
            Move::Left(piece) => {
                let first_piece = self.pieces.first().unwrap();
                if piece.right == first_piece.left {
                    Some(Move::Left(piece))
                } else if piece.left == first_piece.left {
                    Some(Move::Left(piece.reverse()))
                } else {
                    None
                }
            },
            Move::Right(piece) => {
                let last_piece = self.pieces.last().unwrap();
                if last_piece.right == piece.left {
                    Some(Move::Right(piece))
                } else if last_piece.right == piece.right {
                    Some(Move::Right(piece.reverse()))
                } else {
                    None
                }
            },
        }
    }

    pub fn apply_move(&mut self, player_move: Move) {
        match player_move {
            Move::Left(piece) => {
                self.pieces.insert(0, piece);
            },
            Move::Right(piece) => {
                self.pieces.push(piece);
            },
        }
    }
}