use crate::helpers::types::{LetterResultColor, Word, WordList, WordResult};

pub fn filter_word_list(
    word_list: &WordList,
    last_result: &WordResult,
    last_guess: Word,
) -> WordList {
    let filtered_words = word_list
        .iter()
        .filter(|&original_word| {
            let word = original_word.clone();

            if last_guess == word {
                return false;
            }

            let mut word_array = word.chars().collect::<Vec<char>>();

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

            // must have all green letters
            for result in last_result {
                if result.color == LetterResultColor::Yellow {
                    if !word_array.contains(&result.letter) {
                        return false;
                    } else {
                        if word_array[result.position] == result.letter {
                            return false;
                        } else {
                            word_array[result.position] = '-';
                        }
                    }
                }
            }

            // must have all green letters
            for result in last_result {
                if result.color == LetterResultColor::Black {
                    if word_array.contains(&result.letter) {
                        return false;
                    } else {
                        word_array[result.position] = '-';
                    }
                }
            }

            return true;
        })
        .map(|x| x.clone())
        .collect::<Vec<Word>>();

    return filtered_words;
}
