use crate::board::Board;
use crate::board::Move;
use crate::hand::Hand;
use crate::strategy::Strategy;

pub struct Player {
    strategy: Box<dyn Strategy>,
    pub score: usize,
    pub hand: Hand,
}

impl Player {
    pub fn new(strategy: Box<dyn Strategy>) -> Self {
        Player {
            strategy,
            score: 0,
            hand: Hand::empty(),
        }
    }

    pub fn can_move(&self, board: &Board) -> bool {
        board.valid_moves(&self.hand).len() > 0
    }

    pub fn force_move(&mut self, board: &Board, suggested_move: Move) -> Move {
        let player_move = board.validate_move(suggested_move).unwrap();
        self.hand.remove(&player_move.unwrap());
        player_move
    }

    pub fn play_move(&mut self, board: &Board) -> Move {
        let suggested_move = self.strategy.suggest_move(&self.hand, board);
        self.force_move(board, suggested_move)
    }

    pub fn notify_move(&mut self, index: usize, player_move: Move) {
        self.strategy.notify_move(index, player_move);
    }

    pub fn notify_skip(&mut self, index: usize) {
        self.strategy.notify_skip(index);
    }
}