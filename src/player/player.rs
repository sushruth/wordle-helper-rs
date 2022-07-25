use crate::{
    game::Game,
    helpers::{
        logger::Logger,
        pretty_print::{pretty_print_result, pretty_print_word},
        types::{LetterResult, LetterResultColor, Word, WordList, WordResult},
        words::PROBLEMS,
    },
};

use super::{filter::filter_word_list, guesser::guess_next_word, online::ask_online_result};

pub struct Player {
    attempts: u32,
    filtered_words: WordList,
    green_results: Vec<LetterResult>,
    guessed_word_history: WordList,
    last_result: WordResult,
}

impl Player {
    pub fn new() -> Player {
        return Player {
            attempts: 0,
            filtered_words: PROBLEMS.clone().to_vec(),
            green_results: Vec::new(),
            guessed_word_history: WordList::new(),
            last_result: WordResult::new(),
        };
    }

    pub fn single_round(&mut self, word: &Word, game: &Game, logger: &Logger) {
        self.attempts += 1;
        self.guessed_word_history.push(word.to_string());

        let result = game.is_this_the_word(word);

        let _ = &self.handle_result(word, result, logger);
    }

    pub fn handle_result(
        &mut self,
        word: &Word,
        result: Result<Vec<LetterResult>, &str>,
        logger: &Logger,
    ) {
        match result {
            Ok(word_result) => {
                for result in &word_result {
                    if result.color == LetterResultColor::Green {
                        self.green_results.push(result.clone());
                    }
                }

                self.last_result = word_result;

                pretty_print_result(&self.last_result, logger);

                self.filtered_words =
                    filter_word_list(&self.filtered_words, &self.last_result, &word);
            }
            Err(error) => {
                println!("{}", error);
            }
        };
    }

    pub fn single_round_online(&mut self, word: &Word, logger: &Logger) {
        self.attempts += 1;
        self.guessed_word_history.push(word.to_string());

        let result = ask_online_result(logger, word);

        let _ = &self.handle_result(word, result, logger);
    }

    fn is_game_over(&self) -> bool {
        let is_result_empty = self.last_result.len() == 0;
        let is_goal_reached = self
            .last_result
            .iter()
            .all(|r| r.color == LetterResultColor::Green);
        return !is_result_empty && is_goal_reached;
    }

    pub fn play_game(&mut self, game: &Game, logger: &Logger) {
        while !self.is_game_over() {
            let next_guess = guess_next_word(
                &self.filtered_words,
                &self.guessed_word_history,
                &self.green_results,
            );

            self.single_round(&next_guess, game, logger);
        }
    }

    pub fn play_game_online(&mut self, logger: &Logger) {
        while !self.is_game_over() {
            let next_guess = guess_next_word(
                &self.filtered_words,
                &self.guessed_word_history,
                &self.green_results,
            );

            logger.log("Next guess ->");

            pretty_print_word(&next_guess, logger);

            self.single_round_online(&next_guess, logger);
        }
    }
}
