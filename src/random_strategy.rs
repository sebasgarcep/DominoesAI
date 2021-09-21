use rand::Rng;
use rand::rngs::ThreadRng;

use crate::board::Board;
use crate::board::Move;
use crate::hand::Hand;
use crate::strategy::Strategy;

pub struct RandomStrategy {
    rng: ThreadRng,
}

impl RandomStrategy {
    pub fn new() -> Self {
        RandomStrategy {
            rng: rand::thread_rng(),
        }
    }
}

impl Strategy for RandomStrategy {
    fn suggest_move(&mut self, hand: &Hand, board: &Board) -> Move {
        let valid_moves = board.valid_moves(hand);
        let index = self.rng.gen_range(0..valid_moves.len());
        valid_moves[index]
    }

    fn notify_move(&mut self, _: usize, _: Move) {}
    
    fn notify_skip(&mut self, _: usize) {}
}