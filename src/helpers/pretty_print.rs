use std::fmt::Write;
use termion::color::{self, Bg, Black, Green, Reset, Yellow};

use super::{
    logger::Logger,
    types::{LetterResult, LetterResultColor, WordResult},
};

fn colored_letter(letter_result: &LetterResult) -> String {
    match letter_result.color {
        LetterResultColor::Green => {
            format!(
                "{} {} {}",
                Bg(Green),
                letter_result.letter.to_uppercase(),
                Bg(Reset)
            )
        }
        LetterResultColor::Black => {
            format!(
                "{} {} {}",
                Bg(Black),
                letter_result.letter.to_uppercase(),
                Bg(Reset)
            )
        }
        LetterResultColor::Yellow => {
            format!(
                "{} {} {}",
                Bg(Yellow),
                letter_result.letter.to_uppercase(),
                Bg(Reset)
            )
        }
    }
}

pub fn pretty_print_result(result: &WordResult, logger: &Logger) {
    let mut str = String::new();

    for letter_result in result {
        let _ = write!(&mut str, "{}", colored_letter(letter_result));
    }

    logger.log(&format!("{}{}", str, color::Bg(color::Reset)))
}

pub fn pretty_print_word(word: &str, logger: &Logger) {
    let mut str = String::new();

    for letter in word.chars() {
        let _ = write!(&mut str, " {} ", letter.to_uppercase());
    }

    logger.log(&str.to_string());
}
