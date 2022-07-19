use termion::color;

use crate::{
    logger::LOGGER,
    types::{LetterResultColor, WordResult},
};

pub fn pretty_print_result(result: WordResult) {
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

    LOGGER.log(&format!("{}{}", str, color::Bg(color::Reset)))
}

pub fn pretty_print_word(word: &str) {
    let mut str = String::new();

    for letter in word.chars() {
        str += &format!(" {} ", letter.to_uppercase());
    }

    LOGGER.log(&format!("{}", str));
}
