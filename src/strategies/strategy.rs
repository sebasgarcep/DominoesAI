use crate::game::{Board, Hand, Move};

pub trait Strategy {
    fn suggest_move(&mut self, hand: &Hand, board: &Board) -> Move;

    fn notify_start_round(&mut self, index: usize, round: usize, starting_player: usize, scores: &Vec<usize>);

    fn notify_move(&mut self, index: usize, player_move: Move);
    
    fn notify_skip(&mut self, index: usize);
}