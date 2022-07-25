use crate::helpers::{
    logger::Logger,
    types::{LetterResult, LetterResultColor, Word, WordResult},
};

pub fn ask_online_result<'a>(logger: &'a Logger, word: &'a Word) -> Result<WordResult, &'a str> {
    let mut input = String::new();
    let mut result: WordResult = Vec::new();
    let word_array = word.chars().collect::<Vec<char>>();

    'outer: while input.len() != 5 {
        logger.log("❯ enter your online result : ");
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => {
                let input_chars = input.trim().chars().collect::<Vec<char>>();

                for position in 0..5 {
                    let letter = word_array[position];
                    let result_color = input_chars[position];

                    let color = match result_color {
                        'G' | 'g' => LetterResultColor::Green,
                        'B' | 'b' => LetterResultColor::Black,
                        'Y' | 'y' => LetterResultColor::Yellow,
                        _ => {
                            logger.log(&format!("❯ invalid input {}!", letter));
                            continue 'outer;
                        }
                    };

                    result.push(LetterResult {
                        color,
                        letter,
                        position,
                    });
                }

                if result.len() == 5 {
                    break 'outer;
                }
            }
            Err(_) => return Err(&"Error reading input"),
        }
    }

    return Ok(result);
}
