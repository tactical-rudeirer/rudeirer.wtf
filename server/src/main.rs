#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket_contrib;
mod db;
mod models;
mod routes;
mod messages;

use rocket::{get, routes, http::Method};
use rocket_cors::{AllowedHeaders, AllowedOrigins, Error};

#[get("/")]
fn index() -> &'static str {
    "Welcome to the official rudeirer.wtf REST API. If you have further questions on how to use this API please contact Rudi Rudeirer. Expect 'Was?' as an answer!"
}

fn main() -> Result<(), Error> {
    let allowed_origins = AllowedOrigins::All; // might change that to rudeirer.wtf later
    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    }.to_cors()?;

    //println!("Hello, world!");
    rocket::ignite()
        .attach(db::RudiDBConn::fairing())
        .attach(cors)
        .mount(
            "/",
            routes![
                index,
                routes::news::getnews,
                routes::news::createnews,
                routes::compilerbau2vorschlaege::get_comp2,
                routes::compilerbau2vorschlaege::set_comp2,
                routes::quiz::getquiz,
                routes::quiz::validate_quiz,
                routes::quiz::get_highscore
            ],
        )
        .launch();
    Ok(())
}
