use chrono::{NaiveDate, Utc};
use diesel::prelude::*;
use diesel::{Insertable, PgConnection, Queryable};
use rocket::request::FromForm;
use serde::{Deserialize, Serialize};
//use crate::models::schema::news;
use super::schema::{news, users,};
use super::user::{User};

#[derive(Serialize, Deserialize, FromForm, Insertable, Debug)]
#[table_name = "news"]
pub struct NewNews {
    pub title: String,
    pub content: String,
    pub author_id: i32,
}

#[derive(Serialize, Deserialize, Queryable, Debug)]
pub struct News {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub date: NaiveDate,
    pub author_id: i32,
}

// Model for client
#[derive(Serialize, Deserialize, Queryable, Debug)]
pub struct NewsItem {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub date: NaiveDate,
    pub author: String,
}

impl NewsItem {
    // TODO: pagination
    pub fn load(conn: &PgConnection) -> Result<Vec<Self>, diesel::result::Error> {
        let news: Vec<(News, User)> =  news::table
            .inner_join(users::table)
            .load(conn)?;
        return Ok(news.into_iter().map(|(news, user)| 
            NewsItem{
                id: news.id,
                title: news.title,
                content: news.content,
                date: news.date,
                author: user.displayname.unwrap_or("unknown".to_string()),
            }
        ).collect());
    }
}

pub fn example_news() -> News {
    News {
        id: 1,
        title: "test".to_string(),
        content: "test".to_string(),
        date: Utc::now().naive_utc().date(),
        author_id: 3,
    }
}

pub fn load_news(con: &PgConnection) -> () {
    let result: Vec<News> = news::table.load(con).unwrap();
    println!("{:?}", result);
}
