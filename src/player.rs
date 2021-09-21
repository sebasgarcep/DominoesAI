use crate::board::Board;
use crate::board::Move;
use crate::piece::Piece;
use crate::strategy::Strategy;

pub struct Player {
    strategy: Box<dyn Strategy>,
    pub score: usize,
    hand: Vec<Piece>,
}

impl Player {
    pub fn new(strategy: Box<dyn Strategy>) -> Player {
        Player {
            strategy,
            score: 0,
            hand: vec![],
        }
    }

    pub fn hand_size(&self) -> usize {
        self.hand.len()
    }

    pub fn hand_value(&self) -> usize {
        self.hand.iter().map(|&p| p.value()).sum()
    }

    pub fn can_move(&self, board: &Board) -> bool {
        for piece in self.hand.iter() {
            if let Some(_) = board.validate_move(Move::Left(*piece)) {
                return true
            }
            if let Some(_) = board.validate_move(Move::Right(*piece)) {
                return true
            }
        }

        false
    }

    pub fn play_move(&self, board: &Board) -> Move {
        let suggested_move = self.strategy.suggest_move(&self.hand, board);
        board.validate_move(suggested_move).unwrap()
    }

    pub fn notify_move(&self, index: usize, player_move: Move) {
        self.strategy.notify_move(index, player_move);
    }

    pub fn notify_skip(&self, index: usize) {
        self.strategy.notify_skip(index);
    }
}