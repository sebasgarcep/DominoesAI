use crate::game::{Hand, Move};

pub trait Logger {
    fn notify_initial_state(&mut self, round: usize, starting_player: usize, hands: Vec<Hand>);
    fn notify_move(&mut self, index: usize, player_move: Move);
    fn notify_skip(&mut self, index: usize);
    fn notify_round_winner(&mut self, index: usize, round_score: usize, winner_score: usize);
    fn notify_game_winner(&mut self, index: usize);
}