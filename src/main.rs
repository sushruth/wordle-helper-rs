mod game;
mod helpers;
mod player;

use clap::Parser;
use std::time::Instant;

use crate::{game::Game, helpers::logger::Logger, player::player::Player};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The name of the person to greet.
    #[clap(short, long, parse(try_from_str))]
    word: Option<String>,

    /// Whether to print the greeting or not.
    #[clap(short, long)]
    silent: bool,
}

fn main() {
    let now = Instant::now();
    let args = Args::parse();
    let logger = Logger {
        enabled: !args.silent,
    };

    logger.log("---------------");
    let game = Game::new(&args.word, &logger);
    logger.log("---------------");

    let mut player = Player::new();

    player.play_game(&game, &logger);

    let elapsed_time = now.elapsed();
    println!("took {} milliseconds.", elapsed_time.as_millis());
}
