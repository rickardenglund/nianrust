use serde::Deserialize;

#[derive(Clone, PartialEq, Deserialize)]
pub struct Day {
    #[serde(rename = "Letters")]
    pub letters: String,
    #[serde(rename = "PuzzleNumber")]
    pub puzzle_number:u32,
    #[serde(rename = "NWords")]
    pub n_words:u32,
}


pub fn new()->Day {
    Day{
        letters:"".to_owned(),
        puzzle_number: 0,
        n_words:0,
    }
}