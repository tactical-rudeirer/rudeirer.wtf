use crate::db::RudiDBConn;
use crate::messages::response::*;
use crate::models::quiz::{
    load_quiz_from_file, Highscore, Quiz, QuizChallenge, QuizResponse, QuizResult,
};
use crate::models::schema::wasisstrudihighscore;
use diesel::insert_into;
use diesel::prelude::*;
use rand::seq::SliceRandom;
use rocket::{get, post};
use rocket_contrib::json::Json;
use std::collections::HashMap;

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
pub fn validate_quiz(
    conn: RudiDBConn,
    quizdata: Json<QuizResponse>,
) -> ApiResponse<QuizResult, ()> {
    let mut score = 100;
    let mut answered_items = HashMap::new();

    let quiz = load_quiz_from_file(QUIZ_FILE);
    let user_quiz = quizdata.into_inner();
    for (item, user_score) in user_quiz.quiz {
        let correct_score = quiz[&item];
        score -= (correct_score - user_score).abs();
        answered_items.insert(item, correct_score);
    }

    respond_success(QuizResult {
        score,
        quiz: answered_items,
    })
    // TODO: Insert into highscore + give rudicoins
}

#[get("/quiz/highscore")]
pub fn get_highscore(conn: RudiDBConn) -> ApiResponse<Highscore, String> {
    let highscore: Highscore = wasisstrudihighscore::table
        .order((
            wasisstrudihighscore::score.desc(),
            wasisstrudihighscore::date.desc(),
        ))
        .limit(10)
        .load(&conn.0)?;
    respond_success(highscore)
}
