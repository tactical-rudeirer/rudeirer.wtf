use chrono::{NaiveDate, Utc};
use diesel::prelude::*;
use diesel::{Insertable, PgConnection, Queryable};
use rocket::request::FromForm;
use serde::{Deserialize, Serialize};
//use crate::models::schema::news;
use super::schema::news;

#[derive(Serialize, Deserialize, FromForm, Insertable, Debug)]
#[table_name = "news"]
pub struct NewNews {
    pub title: Option<String>,
    pub content: Option<String>,
    pub author_id: Option<i32>,
}

#[derive(Serialize, Deserialize, Queryable, Debug)]
pub struct News {
    pub id: i32,
    pub title: Option<String>,
    pub content: Option<String>,
    pub date: Option<NaiveDate>,
    pub author_id: Option<i32>,
}

pub fn example_news() -> News {
    News {
        id: 1,
        title: Some("test".to_string()),
        content: Some("test".to_string()),
        date: Some(Utc::now().naive_utc().date()),
        author_id: Some(3),
    }
}

pub fn load_news(con: &PgConnection) -> () {
    let result: Vec<News> = news::table.load(con).unwrap();
    println!("{:?}", result);
}
