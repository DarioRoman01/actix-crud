#[macro_use]
extern crate diesel;

pub mod schema;
pub mod models;
pub mod actions;

use self::models::*;
use self::actions::*;


use dotenv::dotenv;
use std::env;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use actix_web::{get, post, put, delete, web, App, Error, HttpServer, Responder, HttpResponse};

type DBpool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[get("/")]
async fn index() -> impl Responder {
    "Hello world!"
}

#[post("/echo")]
async fn echo(payload: web::Json<Post>) -> impl Responder {
    HttpResponse::Ok().json(payload)
}

#[get("/posts")]
async fn list_posts_handler(db_pool: web::Data<DBpool>) -> Result<HttpResponse, Error> {
    let conn = db_pool.get().expect("couldn't get db connection from pool");

    let ps = web::block(move || list_posts(&conn))
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(ps))
}

#[get("/posts/{search_id}")]
async fn get_post_handler(db_pool: web::Data<DBpool>, search_id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let post = web::block(move || {
        let conn = db_pool.get()?;
        get_post(&conn, search_id.into_inner())
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    match post {
        Some(p) => Ok(HttpResponse::Ok().json(p)),
        None => Ok(HttpResponse::NotFound().json(JsonError{
            message: "Post not found".to_string(),
        })),
    }
}

#[post("/posts")]
async fn insert_post_handler(db_pool: web::Data<DBpool>, new_post: web::Json<NewPost>) -> Result<HttpResponse, Error> {
    let conn = db_pool.get().expect("couldn't get db connection from pool");

    let p = web::block(move || insert_post(&conn, &*new_post))
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(p))
}

#[delete("/posts/{search_id}")]
async fn delete_post_handler(db_pool: web::Data<DBpool>, search_id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let conn = db_pool.get().expect("couldn't get db connection from pool");

    let post = web::block(move || delete_post(&conn, search_id.into_inner()))
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(post))
}

#[put("/posts/{search_id}")]
async fn update_post_handler(db_pool: web::Data<DBpool>, search_id: web::Path<i32>, new_post: web::Json<NewPost>) -> Result<HttpResponse, Error> {
    let conn = db_pool.get().expect("couldn't get db connection from pool");

    let post = web::block(move || update_post(&conn, search_id.into_inner(), &new_post))
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(post))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
        .app_data(web::Data::new(pool.clone()))
        .service(list_posts_handler)
        .service(get_post_handler)
        .service(insert_post_handler)
        .service(delete_post_handler)
        .service(update_post_handler)        
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
