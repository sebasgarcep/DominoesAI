mod board;
mod console_logger;
mod game;
mod hand;
mod heuristic_strategy;
mod logger;
mod noop_logger;
mod piece;
mod player;
mod random_strategy;
mod strategy;

use crate::noop_logger::NoopLogger;
use crate::game::Game;
use crate::heuristic_strategy::HeuristicStrategy;
use crate::random_strategy::RandomStrategy;
use crate::strategy::Strategy;

fn main() {
    let num_games = 10000;
    let mut wincount = vec![0; 4];

    for _ in 0..num_games {
        let strategies: Vec<Box<dyn Strategy>> = vec![
            Box::new(HeuristicStrategy::new()),
            Box::new(RandomStrategy::new()),
            Box::new(RandomStrategy::new()),
            Box::new(RandomStrategy::new()),
        ];
    
        let mut game = Game::new(strategies, Box::new(NoopLogger::new()));
    
        let winner_index = game.play();

        wincount[winner_index] += 1;
    }

    for index in 0..4 {
        println!(
            "Player {} won {} / {} = {:.2}% games",
            index,
            wincount[index],
            num_games,
            ((wincount[index] as f64) / (num_games as f64)) * 100.0,
        );
    }
}
