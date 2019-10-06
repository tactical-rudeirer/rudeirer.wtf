use crate::db::RudiDBConn;
use crate::models::schema::news;
use crate::models::{news::NewNews, news::News, news::NewsItem};
use diesel::insert_into;
use diesel::prelude::*;
use rocket::{get, post};
use rocket_contrib::json::Json;
use crate::messages::response::*;

#[get("/news")]
pub fn getnews(conn: RudiDBConn) -> ApiResponse<Vec<NewsItem>, String> {
    let result = NewsItem::load(&conn.0)?;
    respond_success(result)
}


#[post("/news", data = "<newsdata>")]
pub fn createnews(conn: RudiDBConn, newsdata: Json<NewNews>) -> ApiResponse<News, String> {
    return respond_unauthorized();

    let newnews: NewNews = newsdata.into_inner();
    let inserted: News = insert_into(news::table)
        .values(newnews)
        .get_result(&conn.0)
        .unwrap();
    respond_success(inserted)
}
