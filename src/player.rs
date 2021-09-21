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
    pub fn new(strategy: Box<dyn Strategy>) -> Self {
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
        board.valid_moves(&self.hand).len() > 0
    }

    pub fn play_move(&mut self, board: &Board) -> Move {
        let suggested_move = self.strategy.suggest_move(&self.hand, board);
        board.validate_move(suggested_move).unwrap()
    }

    pub fn notify_move(&mut self, index: usize, player_move: Move) {
        self.strategy.notify_move(index, player_move);
    }

    pub fn notify_skip(&mut self, index: usize) {
        self.strategy.notify_skip(index);
    }
}