use crate::game::{Hand, Move};

use super::logger::Logger;

pub struct ConsoleLogger {}

impl ConsoleLogger {
    pub fn new() -> Self {
        ConsoleLogger {}
    }
}

impl Logger for ConsoleLogger {
    fn notify_initial_state(&mut self, round: usize, starting_player: usize, hands: Vec<Hand>) {
        println!("Starting round: {}", round);
        println!("Starting player: {}", starting_player);
        for i in 0..4 {
            println!("Player {}'s Hand", i);
            for piece in hands[i].pieces.iter() {
                print!("({}, {}) ", piece.left, piece.right)
            }
            print!("\n")
        }
        println!("Starting round:");
        println!("##################################################");
    }

    fn notify_move(&mut self, index: usize, player_move: Move) {
        println!("Player {} plays ({}, {})", index, player_move.unwrap().left, player_move.unwrap().right);
    }

    fn notify_skip(&mut self, index: usize) {
        println!("Player {} skips", index);
    }

    fn notify_round_winner(&mut self, index: usize, round_score: usize, winner_score: usize) {
        println!("##################################################");
        println!("Round Winner: {}", index);
        println!("Round Score: {}", round_score);
        println!("Winner Score: {}", winner_score);
        println!("--------------------------------------------------");
    }

    fn notify_game_winner(&mut self, index: usize) {
        println!("Game Winner: {}", index);
    }
}