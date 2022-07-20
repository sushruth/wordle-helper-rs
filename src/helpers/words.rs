use lazy_static::lazy_static;

use crate::helpers::types::WordList;

const PROBLEMS_JSON: &str = include_str!("../../data/problems.json");
const DICTIONARY_JSON: &str = include_str!("../../data/dictionary.json");

lazy_static! {
    pub static ref PROBLEMS: WordList = serde_json::from_str(PROBLEMS_JSON).unwrap();
    pub static ref DICTIONARY: WordList = serde_json::from_str(DICTIONARY_JSON).unwrap();
}
