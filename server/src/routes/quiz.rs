use crate::db::RudiDBConn;
use crate::models::quiz::{load_quiz_from_file, Highscore, Quiz, QuizChallenge, QuizResponse};
use diesel::insert_into;
use diesel::prelude::*;
use rand::seq::SliceRandom;
use rocket::{get, post};
use rocket_contrib::json::Json;
use std::collections::HashMap;
use crate::messages::response::*;

const QUIZ_FILE: &str = "data/wasmagrudi";

#[get("/quiz")]
pub fn getquiz() -> ApiResponse<QuizChallenge, ()> {
    let quiz = load_quiz_from_file(QUIZ_FILE);
    let names: Vec<String> = quiz.into_iter().map(|(name, _score)| name).collect();
    let mut rng = rand::thread_rng();
    let challenge = names
        .choose_multiple(&mut rng, 10)
        .cloned()
        .collect::<Vec<String>>();
    respond_success(challenge)
}

#[post("/quiz", data = "<quizdata>")]
pub fn validate_quiz(conn: RudiDBConn, quizdata: Json<QuizResponse>) -> ApiResponse<Quiz, ()> {
    let result = HashMap::new();
    respond_success(result)
}

#[get("/quiz/highscore")]
pub fn gethighscore(conn: RudiDBConn) -> ApiResponse<Highscore, ()> {
    respond_success(vec![])
}
