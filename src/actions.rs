use diesel::prelude::*;

use crate::models;

type DbError = Box<dyn std::error::Error + Send + Sync>;

/// Create a new post in the database.
pub fn insert_post(conn: &PgConnection, new_post: &models::NewPost) -> Result<models::Post, DbError> {
    use crate::schema::posts;

    diesel::insert_into(posts::table)
        .values(new_post)
        .get_result(conn)
        .map_err(|e| e.into())    
}

/// Get all posts from the database.
pub fn list_posts(conn: &PgConnection) -> Result<Vec<models::Post>, DbError> {
    use crate::schema::posts;
    posts::table.load::<models::Post>(conn).map_err(|e| e.into())
}

/// Get a post from the database.
pub fn get_post(conn: &PgConnection, search_id: i32) -> Result<Option<models::Post>, DbError> {
    use crate::schema::posts::dsl::*;
    let post = posts.filter(id.eq(search_id)).first::<models::Post>(conn).optional()?;
    Ok(post)
}

/// Delete a post from the database.
pub fn delete_post(conn: &PgConnection, id: i32) -> Result<models::Post, DbError> {
    use crate::schema::posts;
    diesel::delete(posts::table.find(id)).get_result(conn).map_err(|e| e.into())
}

/// Update a post in the database.
pub fn update_post(conn: &PgConnection, update_id: i32, new_post: &models::NewPost) -> Result<models::Post, DbError> {
    use crate::schema::posts::dsl::*;
    diesel::update(posts.find(update_id))
        .set((
            title.eq(&new_post.title),
            slug.eq(&new_post.slug),
            body.eq(&new_post.body),
        ))
        .get_result(conn)
        .map_err(|e| e.into())
}