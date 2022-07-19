mod types;
mod words;

#[path = "game/evaluate.rs"]
mod evaluate;

#[path = "helpers/pretty_print_result.rs"]
mod pretty_print_result;

use crate::{evaluate::Game, pretty_print_result::pretty_print_result};
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The name of the person to greet.
    #[clap(short, long, parse(try_from_str))]
    word: String,
}

fn main() {
    let args = Args::parse();

    let game = Game::new(Some(&args.word));

    match game.is_this_the_word(&"might".to_string()) {
        Ok(result) => pretty_print_result(result),
        Err(err) => println!("Error: {}", err),
    }
}
