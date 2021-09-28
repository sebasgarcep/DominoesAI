use crate::game::{Hand, Move};

use super::logger::Logger;

pub struct NoopLogger {}

impl NoopLogger {
    pub fn new() -> Self {
        NoopLogger {}
    }
}

impl Logger for NoopLogger {
    fn notify_initial_state(&mut self, _: usize, _: usize, _: Vec<Hand>) {}
    fn notify_move(&mut self, _: usize, _: Move) {}
    fn notify_skip(&mut self, _: usize) {}
    fn notify_round_winner(&mut self, _: usize, _: usize, _: usize) {}
    fn notify_game_winner(&mut self, _: usize) {}
}