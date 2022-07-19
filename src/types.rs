pub type Word = String;
pub type Letter = char;
// pub type Score = i32;
pub type WordList = Vec<Word>;

#[derive(Clone, Debug)]
pub enum LetterResultColor {
    Green,
    Yellow,
    Black,
}

#[derive(Clone, Debug)]
pub struct LetterResult {
    pub color: LetterResultColor,
    pub letter: Letter,
    pub position: usize,
}

pub type WordResult = Vec<LetterResult>;
