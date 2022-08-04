#[macro_use]
extern crate diesel;

pub mod schema;
pub mod models;

use dotenv::dotenv;
use std::env;
use diesel::prelude::*;
use diesel::pg::PgConnection;

fn main() {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let conn = PgConnection::establish(&db_url).expect("Error connecting to database");

    use self::schema::posts::dsl::*;

    let ps = posts.load::<models::Post>(&conn).expect("Error loading posts");
    println!("Displaying {} posts", ps.len());
}
