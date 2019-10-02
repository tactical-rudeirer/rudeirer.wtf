use diesel::PgConnection;
use rocket_contrib::database;

#[database("rudidb")]
pub struct RudiDBConn(PgConnection);
