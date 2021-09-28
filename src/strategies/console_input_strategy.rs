use std::io::stdin;

use crate::game::{Board, Hand, Move};

use super::strategy::Strategy;

pub struct ConsoleInputStrategy {}

impl ConsoleInputStrategy {
    pub fn new() -> Self {
        ConsoleInputStrategy {}
    }
}

impl ConsoleInputStrategy {
    fn ask_move(&self, hand: &Hand, board: &Board) -> Move {
        loop {
            println!("Enter the piece you want to use (1-{}):", hand.size());
            let mut piece_input = String::new();
            if let Result::Err(_) = stdin().read_line(&mut piece_input) {
                println!("Failed to read the piece number.");
                continue;
            }
            let piece_number: usize;
            match piece_input.trim().parse() {
                Result::Ok(value) => { piece_number = value; }
                Result::Err(_) => {
                    println!("You must enter a valid piece number.");
                    continue;
                }
            }
            if piece_number < 1 || piece_number > hand.size() {
                println!("The piece number must be in the range 1-{}.", hand.size());
                continue;
            }
            println!("Choose where to play the piece: (1 = Left, 2 = Right):");
            let mut side_input = String::new();
            if let Result::Err(_) = stdin().read_line(&mut side_input) {
                println!("Failed to read the side.");
                continue;
            }
            let side_number: usize;
            match side_input.trim().parse() {
                Result::Ok(value) => { side_number = value; }
                Result::Err(_) => {
                    println!("You must enter a valid side number.");
                    continue;
                }
            }
            if side_number != 1 && side_number != 2 {
                println!("The side number must be either 1 or 2.");
                continue;
            }
            let chosen_piece = hand.pieces[piece_number - 1];
            let potential_move = if side_number == 1 {
                Move::Left(chosen_piece)
            } else {
                Move::Right(chosen_piece)
            };

            if let Some(player_move) = board.validate_move(potential_move) {
                return player_move;
            } else {
                println!("This is an invalid move.");
            }
        }
    }
}

impl Strategy for ConsoleInputStrategy {
    fn suggest_move(&mut self, hand: &Hand, board: &Board) -> Move {
        println!("--------------------------------------------------");
        println!("Board State:");
        for piece in board.pieces.iter() {
            print!("({}, {}) ", piece.left, piece.right);
        }
        println!();
        println!("--------------------------------------------------");
        println!("Hand State:");
        for (index, piece) in hand.pieces.iter().enumerate() {
            println!("{}] ({}, {})", index + 1, piece.left, piece.right);
        }
        println!();
        self.ask_move(hand, board)
    }

    fn notify_start_round(&mut self, index: usize, round: usize, starting_player: usize, scores: &Vec<usize>) {
        println!("You are player {}", index);
        println!("Current scores:");
        for (position, score) in scores.iter().enumerate() {
            println!("Player {}: {}", position, score);
        }
        println!("Starting round {}", round);
        println!("Player {} plays first", starting_player);
        println!("--------------------------------------------------");
    }

    fn notify_move(&mut self, index: usize, player_move: Move) {
        println!("Player {} plays ({}, {})", index, player_move.unwrap().left, player_move.unwrap().right);
    }
    
    fn notify_skip(&mut self, index: usize) {
        println!("Player {} skips", index);
    }
}