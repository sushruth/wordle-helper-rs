use crate::helpers::types::{LetterResultColor, Word, WordList, WordResult};

pub fn filter_word_list(
    word_list: &WordList,
    last_result: &WordResult,
    last_guess: &Word,
) -> WordList {
    let filtered_words = word_list
        .iter()
        .filter(|&original_word| {
            if last_guess == original_word {
                return false;
            }

            let mut word_array = original_word.chars().collect::<Vec<char>>();

            // must have all green letters
            for result in last_result {
                if result.color == LetterResultColor::Green {
                    if word_array[result.position] != result.letter {
                        return false;
                    } else {
                        word_array[result.position] = '-';
                    }
                }
            }

            // must have yellow letters in different positions
            for result in last_result {
                if result.color == LetterResultColor::Yellow {
                    if !word_array.contains(&result.letter)
                        || word_array[result.position] == result.letter
                    {
                        return false;
                    } else {
                        for i in 0..word_array.len() {
                            if word_array[i] == result.letter {
                                word_array[i] = '-';
                            }
                        }
                    }
                }
            }

            // must NOT have black letters
            for result in last_result {
                if result.color == LetterResultColor::Black {
                    if word_array.contains(&result.letter) {
                        return false;
                    } else {
                        for i in 0..word_array.len() {
                            if word_array[i] == result.letter {
                                word_array[i] = '-';
                            }
                        }
                    }
                }
            }

            true
        })
        .cloned()
        .collect::<Vec<Word>>();

    filtered_words
}
