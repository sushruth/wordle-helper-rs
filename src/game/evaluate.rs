use crate::{
    types::{LetterResult, LetterResultColor, Word, WordResult},
    words::{DICTIONARY, PROBLEMS},
};
use rand::seq::SliceRandom;

pub struct Game {
    pub goal_word: Word,
}

impl Game {
    fn new_random() -> Game {
        let mut rng = rand::thread_rng();
        let goal_word = PROBLEMS.choose(&mut rng).unwrap().to_string();
        return Game { goal_word };
    }

    pub fn new(input: Option<&Word>) -> Game {
        return match input {
            Some(word) => Game {
                goal_word: word.to_string(),
            },
            None => Game::new_random(),
        };
    }

    pub fn is_this_the_word(&self, input: &Word) -> Result<WordResult, &str> {
        if !(DICTIONARY.contains(&input) || PROBLEMS.contains(&input)) {
            return Err("Word not in dictionary");
        }

        let mut result: Vec<Option<LetterResult>> = vec![None; input.len()];
        let mut goal_word_copy = self.goal_word.clone();

        for index in 0..input.len() {
            let letter = input.chars().nth(index).unwrap();
            let goal_letter = goal_word_copy.chars().nth(index).unwrap();

            if letter == goal_letter {
                result[index] = Some(LetterResult {
                    color: LetterResultColor::Green,
                    letter,
                    position: index,
                });
                goal_word_copy.replace_range(index..index, "-")
            } else if !self.goal_word.contains(*&letter) {
                result[index] = Some(LetterResult {
                    color: LetterResultColor::Black,
                    letter,
                    position: index,
                });
            }
        }

        for index in 0..input.len() {
            match result[index] {
                None => {
                    let letter = input.chars().nth(index).unwrap();

                    if goal_word_copy.contains(*&letter) {
                        result[index] = Some(LetterResult {
                            color: LetterResultColor::Yellow,
                            letter,
                            position: index,
                        });
                        goal_word_copy = goal_word_copy.replace(&letter.to_string(), "-")
                    } else {
                        result[index] = Some(LetterResult {
                            color: LetterResultColor::Black,
                            letter,
                            position: index,
                        });
                    }
                }
                Some(_) => {}
            }
        }

        return Ok(result.into_iter().filter_map(|x| x).collect());
    }
}
