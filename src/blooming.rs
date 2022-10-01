const BASE_URL: &str = "https://blooming-hollows-80357.herokuapp.com";

pub fn today_url() -> String {
    String::from(format!("{}/today", BASE_URL))
}

pub fn history_url() -> String {
    String::from(format!("{}/history", BASE_URL))
}

use serde::Deserialize;

#[derive(Clone, PartialEq, Deserialize)]
pub struct Day {
    #[serde(rename = "Letters")]
    pub letters: String,
    #[serde(rename = "PuzzleNumber")]
    pub puzzle_number: u32,
    #[serde(rename = "NWords")]
    pub n_words: u32,
}

#[derive(Clone, PartialEq, Deserialize, Debug)]
pub struct HistoryResponse {
    #[serde(rename = "Words")]
    pub days: Vec<HistoryDay>,
}

#[derive(Clone, PartialEq, Deserialize, Debug)]
pub struct HistoryDay {
    #[serde(rename = "Word")]
    pub word: String,
    #[serde(rename = "Shuffle")]
    pub shuffle: String,
    #[serde(rename = "I")]
    pub i: u32,
}
