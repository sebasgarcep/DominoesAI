use rand::RngCore;

use crate::board::Board;
use crate::board::Move;
use crate::hand::Hand;
use crate::piece::Piece;
use crate::player::Player;
use crate::strategy::Strategy;

pub struct Game {
    round: usize,
    players: Vec<Player>,
    winning_score: usize,
    board: Board,
    index: usize,
}

impl Game {
    pub fn new(strategies: Vec<Box<dyn Strategy>>) -> Self {
        if strategies.len() != 4 {
            panic!("You must provide exactly 4 strategies to a game.");
        }

        Game {
            round: 0,
            players: strategies.into_iter().map(|s| Player::new(s)).collect(),
            winning_score: 100,
            board: Board::new(),
            index: 0,
        }
    }

    fn to_next_player(&mut self) {
        self.index = (self.index + 1) % self.players.len();
    }

    fn initialize_hands(&mut self) {
        let mut rng = rand::thread_rng();

        let mut pieces_sortable = vec![];

        for i in 0..=6 {
            for j in i..=6 {
                pieces_sortable.push((Piece::new(i, j), rng.next_u64()));
            }
        }

        pieces_sortable.sort_by(|(_, v1), (_, v2)| v1.cmp(v2));

        let mut pieces: Vec<Piece> = pieces_sortable.into_iter().map(|(p, _)| p).collect();

        for index in 0..4 {
            self.players[index].hand = Hand::new(pieces.split_off((3 - index) * 7));
        }
    }

    fn initialize_first_round(&mut self) {
        if self.round != 1 {
            return;
        }

        // Set initial player
        let starting_piece = Piece::new(6, 6);
        for index in 0..self.players.len() {
            if self.players[index].hand.contains(&starting_piece) {
                self.index = index;
                break;
            }
        }

        // FIXME: Force the first player to play 6/6

        self.to_next_player();
    }

    fn prepare_round(&mut self) {
        self.round += 1;
        self.board = Board::new();
        self.initialize_hands();
        self.initialize_first_round();
    }

    fn notify_move(&mut self, player_move: Move) {
        for player in self.players.iter_mut() {
            player.notify_move(self.index, player_move);
        }
    }

    fn notify_skip(&mut self) {
        for player in self.players.iter_mut() {
            player.notify_skip(self.index);
        }
    }

    fn current_player_can_move(&self) -> bool {
        self.players[self.index].can_move(&self.board)
    }

    fn current_player_move(&mut self) -> Move {
        self.players[self.index].play_move(&self.board)
    }

    fn current_player_won(&self) -> bool {
        self.players[self.index].hand.size() == 0
    }

    fn stalemate(&self, board: &Board) -> bool {
        self.players.iter().all(|p| !p.can_move(board))
    }

    fn current_player_has_smallest_hand(&self) -> bool {
        let current_hand_value = &self.players[self.index].hand.value();
        for index in 0..self.players.len() {
            if index == self.index {
                continue;
            }

            let hand_value = &self.players[index].hand.value();

            if current_hand_value > hand_value {
                return false;
            }
        }

        true
    }

    fn player_with_smallest_hand(&self) -> usize {
        let (index, _) = (0..self.players.len()).fold(None, |acc, index| {
            let hand_value = self.players[index].hand.value();
            match acc {
                None => Some((index, hand_value)),
                Some((acc_index, acc_hand_value)) => {
                    if acc_hand_value <= hand_value {
                        Some((acc_index, acc_hand_value))
                    } else {
                        Some((index, hand_value))
                    }
                },
            }
        }).unwrap();

        index
    }

    fn play_round(&mut self) -> usize {
        loop {
            if self.current_player_can_move() {
                let player_move = self.current_player_move();
                self.board.apply_move(player_move);
                self.notify_move(player_move);

                if self.current_player_won() {
                    return self.index;
                }

                if self.stalemate(&self.board) {
                    if self.current_player_has_smallest_hand() {
                        return self.index;
                    }

                    return self.player_with_smallest_hand();
                }
            } else {
                self.notify_skip();
            }

            self.to_next_player();
        }
    }

    fn winner_score(&self, winner_index: usize) -> usize {
        let mut result = 0;
        for index in 0..self.players.len() {
            if index == winner_index { continue; }
            result += (&self.players[index]).hand.value();
        }
        return result;
    }

    fn set_next_round_starter(&mut self, winner_index: usize) {
        self.index = winner_index;
    }

    pub fn play(&mut self) -> usize {
        loop {
            self.prepare_round();
            let winner_index = self.play_round();

            self.set_next_round_starter(winner_index);
            let score = self.winner_score(winner_index);
            let winner = &mut self.players[winner_index];
            winner.score += score;

            if winner.score >= self.winning_score {
                return winner_index;
            }
        }
    }
}