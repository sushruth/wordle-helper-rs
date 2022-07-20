mod types;
mod words;

#[path = "game/evaluate.rs"]
mod evaluate;

#[path = "helpers/logger.rs"]
mod logger;

#[path = "helpers/pretty_print_result.rs"]
mod pretty_print_result;

use crate::pretty_print_result::pretty_print_result;
use clap::Parser;
use evaluate::Game;
use logger::Logger;
use pretty_print_result::pretty_print_word;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The name of the person to greet.
    #[clap(short, long, parse(try_from_str))]
    word: String,

    /// Whether to print the greeting or not.
    #[clap(short, long)]
    silent: bool,
}

fn main() {
    let args = Args::parse();

    let game = Game::new(Some(&args.word));

    let logger = Logger {
        enabled: !args.silent,
    };

    logger.log("---------------");

    pretty_print_word(&game.goal_word, &logger);

    logger.log("---------------");

    match game.is_this_the_word(&"saree".to_string()) {
        Ok(result) => pretty_print_result(result, &logger),
        Err(err) => println!("Error: {}", err),
    }
}
