use rocket::{get, post};
use rocket::http::Status;
use std::fs::{read_to_string, write};
use crate::messages::response::*;

const COMP2_PATH: &str = "data/compilerbau2.txt";
const LIMIT: usize = 5000;

#[get("/compilerbau2")]
pub fn get_comp2() -> ApiResponse<String, String> {
    let content = read_to_string(COMP2_PATH).map_err(|_| "Unable to open file")?;
    respond_success(content)
}

#[post("/compilerbau2", data = "<comp2>")]
pub fn set_comp2(comp2: String) -> ApiResponse<(), String> {
    if comp2.len() <= LIMIT {
        write(COMP2_PATH, comp2).map_err(|_| "Unable to write data to file")?;
        respond_success(())
    } else {
        respond_error_status(format!("Input is too large. The limit is {}!", LIMIT), Status::BadRequest)
    }
}

