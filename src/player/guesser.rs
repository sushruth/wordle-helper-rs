use std::collections::HashMap;

use crate::helpers::{
    types::{LetterPositionScore, LetterResult, Score, Word, WordList},
    words::{DICTIONARY, PROBLEMS},
};

pub fn guess_next_word(
    filtered_word_list: &WordList,
    guessed_words: &WordList,
    green_results: &Vec<LetterResult>,
) -> String {
    let len = filtered_word_list.len();

    if len == PROBLEMS.len() {
        return "saree".to_string();
    }

    if len == 1 {
        return filtered_word_list[0].to_string();
    }

    let lps = get_lps(filtered_word_list);
    let (problem_score_map, problem_high_score) = get_lps_word_map(&PROBLEMS, &lps, green_results);
    let (dictionary_score_map, dictionary_high_score) =
        get_lps_word_map(&DICTIONARY, &lps, green_results);

    let highest_score_words = {
        if problem_high_score > dictionary_high_score {
            get_highest_score_words(&problem_score_map, &problem_high_score)
        } else {
            get_highest_score_words(&dictionary_score_map, &dictionary_high_score)
        }
    };

    for word in highest_score_words {
        if !guessed_words.contains(&word) {
            return word;
        }
    }

    filtered_word_list[0].to_string()
}

fn get_lps(filtered_word_list: &WordList) -> LetterPositionScore {
    let mut lps: LetterPositionScore = vec![HashMap::new(); 5];

    for word in filtered_word_list {
        let letters = word.clone().chars().collect::<Vec<char>>();
        for index in 0..letters.len() {
            let letter = letters[index];
            let hash_map = &mut lps[index as usize];

            if hash_map.contains_key(&letter) {
                let score = hash_map.get(&letter).unwrap();
                hash_map.insert(letter, score + 1);
            } else {
                hash_map.insert(letter, 1);
            }
        }
    }

    lps
}

fn get_lps_word_map(
    word_list: &WordList,
    lps: &LetterPositionScore,
    green_results: &Vec<LetterResult>,
) -> (HashMap<std::string::String, Score>, Score) {
    let mut score_map: HashMap<Word, Score> = HashMap::new();
    let mut green_position_letters: Vec<Vec<char>> = vec![Vec::new(); 5];
    let mut green_letters: Vec<char> = vec![];
    let mut lps_letters: Vec<char> = vec![];

    for position_score in lps {
        for key in position_score.keys() {
            lps_letters.push(*key);
        }
    }

    for ele in green_results {
        green_position_letters[ele.position].push(ele.letter);
        green_letters.push(ele.letter);
    }

    let mut highest_score = 0;

    for word in word_list {
        let word_array = word.chars().collect::<Vec<char>>();
        let mut score = 0;
        let mut seen_letters: Vec<char> = vec![];

        for i in 0..word_array.len() {
            let letter = word_array[i];

            if green_position_letters[i].contains(&letter) {
                score -= 2;
            } else if green_letters.contains(&letter) {
                score -= 1;
            } else {
                let mut delta = *lps[i].get(&letter).unwrap_or(&0);
                if seen_letters.contains(&letter) {
                    delta -= 2;
                }
                score += delta;
            }

            if lps_letters.contains(&letter) {
                score += 2;
            }

            seen_letters.push(letter);
        }

        if highest_score <= score {
            highest_score = score;
        }

        if score > 0 {
            score_map.insert(word.clone(), score);
        }
    }

    (score_map, highest_score)
}

fn get_highest_score_words(score_map: &HashMap<Word, Score>, highest_score: &Score) -> WordList {
    let mut highest_score_words: WordList = WordList::new();

    for (key, value) in score_map {
        if value == highest_score {
            highest_score_words.push(key.clone());
        }
    }

    highest_score_words
}
