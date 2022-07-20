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
        let mut goal_word_array = self.goal_word.clone().chars().collect::<Vec<char>>();

        for index in 0..input.len() {
            let letter = input.chars().nth(index).unwrap();
            let goal_letter = goal_word_array[index];

            if letter == goal_letter {
                result[index] = Some(LetterResult {
                    color: LetterResultColor::Green,
                    letter,
                    position: index,
                });
                goal_word_array[index] = '-';
            } else if !self.goal_word.contains(*&letter) {
                result[index] = Some(LetterResult {
                    color: LetterResultColor::Black,
                    letter,
                    position: index,
                });
            }
        }

        for index in 0..input.len() {
            let letter = input.chars().nth(index).unwrap();

            match result[index] {
                None if goal_word_array.contains(&letter) => {
                    result[index] = Some(LetterResult {
                        color: LetterResultColor::Yellow,
                        letter,
                        position: index,
                    });

                    for i in 0..goal_word_array.len() {
                        if letter == goal_word_array[i] {
                            goal_word_array[i] = '-';
                        }
                    }
                }
                None => {
                    result[index] = Some(LetterResult {
                        color: LetterResultColor::Black,
                        letter,
                        position: index,
                    });
                }
                Some(_) => {}
            }
        }

        return Ok(result.into_iter().filter_map(|x| x).collect());
    }
}
