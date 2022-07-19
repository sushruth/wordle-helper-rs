use termion::color;

use crate::types::{LetterResultColor, WordResult};

pub fn pretty_print_result(result: WordResult) {
    let mut str = String::new();

    for letter_result in result {
        match letter_result.color {
            LetterResultColor::Green => {
                str += &format!(
                    "{} {} ",
                    color::Fg(color::Green),
                    letter_result.letter.to_uppercase()
                )
            }
            LetterResultColor::Yellow => {
                str += &format!(
                    "{} {} ",
                    color::Fg(color::Yellow),
                    letter_result.letter.to_uppercase()
                )
            }
            LetterResultColor::Black => {
                str += &format!(
                    "{} {} ",
                    color::Fg(color::Black),
                    letter_result.letter.to_uppercase()
                )
            }
        }
    }

    println!("{}", str);
}
