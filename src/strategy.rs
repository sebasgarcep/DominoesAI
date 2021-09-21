use crate::board::Board;
use crate::board::Move;
use crate::hand::Hand;

pub trait Strategy {
    fn suggest_move(&mut self, hand: &Hand, board: &Board) -> Move;

    fn notify_move(&mut self, index: usize, player_move: Move);
    
    fn notify_skip(&mut self, index: usize);
}