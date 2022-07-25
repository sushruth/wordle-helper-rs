use termion::color;

use super::{
    logger::Logger,
    types::{LetterResultColor, WordResult},
};

pub fn pretty_print_result(result: &WordResult, logger: &Logger) {
    let mut str = String::new();

    for letter_result in result {
        match letter_result.color {
            LetterResultColor::Green => {
                str += &format!(
                    "{} {} ",
                    color::Bg(color::Green),
                    letter_result.letter.to_uppercase()
                )
            }
            LetterResultColor::Yellow => {
                str += &format!(
                    "{} {} ",
                    color::Bg(color::Yellow),
                    letter_result.letter.to_uppercase()
                )
            }
            LetterResultColor::Black => {
                str += &format!(
                    "{} {} ",
                    color::Bg(color::Black),
                    letter_result.letter.to_uppercase()
                )
            }
        }
    }
    logger.log(&format!("{}{}", str, color::Bg(color::Reset)))
}

pub fn pretty_print_word(word: &str, logger: &Logger) {
    let mut str = String::new();

    for letter in word.chars() {
        str += &format!(" {} ", letter.to_uppercase());
    }

    logger.log(&str.to_string());
}
