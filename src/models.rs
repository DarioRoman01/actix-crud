use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub body: String,
}

use super::schema::posts;

#[derive(Insertable, Deserialize)]
#[table_name = "posts"]
pub struct NewPost {
    pub title: String,
    pub slug: String,
    pub body: String,
}

#[derive(Serialize)]
pub struct JsonError {
    pub message: String,
}