use crate::db::RudiDBConn;
use crate::models::{user::User};
use crate::models::schema::users;
use diesel::insert_into;
use diesel::prelude::*;
use rocket::http::{Cookies, Cookie, Status};
use rocket::{get, post, response::status};
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};
use crate::messages::response::*;

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}


#[post("/register")]
pub fn register(cookies: Cookies, conn: RudiDBConn) -> ApiResponse<(), String> {
    return respond_unauthorized();
}

#[post("/login", data="<logindata>")]
pub fn login(mut cookies: Cookies, conn: RudiDBConn, logindata: Json<LoginRequest>) -> ApiResponse<(), String> {
    return respond_unauthorized();


    let logindata = logindata.into_inner();
    // todo: check password (how to hash?)
    let result: usize = users::table.filter(users::username.eq(logindata.username)).execute(&conn.0).unwrap();
    if result > 0 {
        let mut new_cookie = Cookie::new("user", "user_id_todo");
        new_cookie.set_secure(true);
        cookies.add_private(new_cookie);
        respond_success(())
    } else {
        respond_error("could not be logged in".to_string())
    } 
}

#[post("/logout")]
pub fn logout(mut cookies: Cookies) -> ApiResponse<(), String> {
    return respond_unauthorized();


    match cookies.get_private("user") {
        Some(user_cookie) => cookies.remove_private(user_cookie),
        None => (),
    };
}
