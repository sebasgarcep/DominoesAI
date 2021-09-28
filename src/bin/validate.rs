use dominoes_ai::game::Game;
use dominoes_ai::loggers::NoopLogger;
use dominoes_ai::strategies::{Strategy, HeuristicStrategy, RandomStrategy};

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
