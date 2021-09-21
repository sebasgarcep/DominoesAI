use crate::board::Board;
use crate::board::Move;
use crate::piece::Piece;

pub trait Strategy {
    fn suggest_move(&mut self, hand: &Vec<Piece>, board: &Board) -> Move;

    fn notify_move(&self, index: usize, player_move: Move);
    
    fn notify_skip(&self, index: usize);
}