use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::read_to_string;

pub type Quiz = HashMap<String, i32>;

pub type QuizChallenge = Vec<String>;

#[derive(Serialize, Deserialize)]
pub struct QuizResponse {
    pub name: Option<String>,
    pub quiz: Quiz,
}

pub type Highscore = Vec<HighscoreItem>;
#[derive(Serialize, Deserialize)]
pub struct HighscoreItem {
    pub name: String,
    pub score: i32,
    pub date: NaiveDate,
}

pub fn load_quiz_from_file(file: &str) -> Quiz {
    let contents = read_to_string(file).unwrap_or_default();
    let mut res: HashMap<String, i32> = HashMap::new();
    for line in contents.lines() {
        if line.trim().is_empty() {
            continue; // skip empty lines
        }
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() != 2 {
            continue; // skip lines that don't conform to 'name: score'
        }
        let name = parts[0];
        let score = parts[0].trim().parse::<i32>().unwrap_or(0); // use 0 if score is not an int
        res.insert(name.to_string(), score);
    }
    res
}
