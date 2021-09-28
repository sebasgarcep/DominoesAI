mod board;
mod console_logger;
mod game;
mod hand;
mod logger;
mod piece;
mod player;
mod random_strategy;
mod strategy;

use crate::console_logger::ConsoleLogger;
use crate::game::Game;
use crate::random_strategy::RandomStrategy;
use crate::strategy::Strategy;

fn main() {
    let strategies: Vec<Box<dyn Strategy>> = vec![
        Box::new(RandomStrategy::new()),
        Box::new(RandomStrategy::new()),
        Box::new(RandomStrategy::new()),
        Box::new(RandomStrategy::new()),
    ];

    let mut game = Game::new(strategies, Box::new(ConsoleLogger::new()));

    game.play();
}
