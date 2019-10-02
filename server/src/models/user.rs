use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use super::schema::users;

#[derive(Serialize, Deserialize, Queryable, Debug)]
pub struct User {
    id: i32,
    username: String,
    password: String,
    displayname: Option<String>,
    role: Option<String>
}

impl User {
    pub fn set_defaults(&mut self) {
        self.displayname.get_or_insert(self.username.clone());
        self.role.get_or_insert("user".to_string());
    }
}

#[derive(Serialize, Deserialize, Insertable, Debug)]
#[table_name = "users"]
pub struct NewUser {
    username: String,
    password: String,
    displayname: Option<String>,
    role: Option<String>
}
