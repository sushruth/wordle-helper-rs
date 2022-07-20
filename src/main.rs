mod game;
mod helpers;
mod player;

use clap::Parser;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::time::Instant;

use crate::{
    game::Game,
    helpers::{logger::Logger, types::Word, words::PROBLEMS},
    player::player::Player,
};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, parse(try_from_str))]
    word: Option<String>,

    #[clap(short, long)]
    silent: bool,

    #[clap(short, long)]
    all: bool,

    #[clap(value_parser, default_value_t = 1)]
    count: usize,
}

fn play_one_game(player: &mut Player, word: &Option<Word>, logger: &Logger) {
    logger.log("---------------");
    let game = Game::new(word, &logger);
    logger.log("---------------");

    player.play_game(&game, &logger);
}

fn main() {
    let now = Instant::now();
    let args = Args::parse();
    let logger = Logger {
        enabled: !args.silent,
    };

    if args.all {
        PROBLEMS.par_iter().for_each(|word| {
            play_one_game(&mut Player::new(), &Some(word.to_string()), &logger);
        });
    } else {
        for _ in 0..args.count {
            play_one_game(&mut Player::new(), &args.word, &logger);
        }
    }

    let elapsed_time = now.elapsed();
    println!("took {} milliseconds.", elapsed_time.as_millis());
}
