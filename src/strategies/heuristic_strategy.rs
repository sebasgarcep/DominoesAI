use crate::game::{Board, Hand, Move};

use super::strategy::Strategy;

pub struct HeuristicStrategy {}

impl HeuristicStrategy {
    pub fn new() -> Self {
        HeuristicStrategy {}
    }
}

impl Strategy for HeuristicStrategy {
    fn suggest_move(&mut self, hand: &Hand, board: &Board) -> Move {
        let valid_moves = board.valid_moves(hand);

        // First try to put down the largest double
        let maybe_double_move: Option<&Move> = valid_moves.iter()
            .filter(|m| m.unwrap().is_double())
            .max_by_key(|m| m.unwrap().value());

        if let Some(player_move) = maybe_double_move {
            return *player_move;
        }

        // Prioritize playing to the pieces with the largest percentage of
        // owned pieces vs total available pieces, unless:
        // - one owns all the pieces
        // - that is the last of that piece the player owns
        // - one owns a double of that number
        let mut hand_count = vec![0; 7];
        let mut available_count = vec![7; 7];
        let mut double_flag = vec![false; 7];

        for piece in hand.pieces.iter() {
            if piece.is_double() {
                hand_count[piece.left] += 1;
                available_count[piece.left] -= 1;
                double_flag[piece.left] = true;
            } else {
                hand_count[piece.left] += 1;
                hand_count[piece.right] += 1;
                available_count[piece.left] -= 1;
                available_count[piece.right] -= 1;
            }
        }

        for piece in board.pieces.iter() {
            if piece.is_double() {
                available_count[piece.left] -= 1;
            } else {
                available_count[piece.left] -= 1;
                available_count[piece.right] -= 1;
            }
        }

        *valid_moves.iter()
            .min_by_key(|m| {
                let exposed_value = m.exposed_value();
                let num_not_owned = available_count[exposed_value] - hand_count[exposed_value];

                let weight =
                    if double_flag[exposed_value] { 2 }
                    else if num_not_owned == 0 || hand_count[exposed_value] == 1 { 1 }
                    else { 0 };

                8 * weight + num_not_owned
            })
            .unwrap()
    }

    // Ignore round starts
    fn notify_start_round(&mut self, _: usize, _: usize, _: usize, _: &Vec<usize>) {}

    // Ignore other player's moves
    fn notify_move(&mut self, _: usize, _: Move) {}
    
    // Ignore skips
    fn notify_skip(&mut self, _: usize) {}
}