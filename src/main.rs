extern crate diesel;
extern crate dotenv;
extern crate r2d2_mysql;

use actix_web::{web, App, HttpServer, Responder};
use dotenv::dotenv;
use std::env;
use r2d2_mysql::mysql::{from_row};

mod db;
mod model;

fn index(info: web::Path<(u32, String)>) -> impl Responder {
    format!("hello {} id: {}", info.1, info.0)
}

fn main() -> std::io::Result<()> {
    // Grab the env vars.
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let pool = db::init_pool(&database_url);

    {
        let mut conn = pool.get().unwrap();
        let posts: Vec<model::Post> = conn.prep_exec("SELECT * FROM post", ()).map(|result| {
            result.map(|x| x.unwrap()).map(|row| {
                let (id, title, body, published) = from_row(row);
                model::Post{
                    id,
                    title,
                    body,
                    published
                }
            }).collect()
        }).unwrap();
        println!("got posts: {:?}", posts);
    }

    HttpServer::new(
        move || App::new()
        .data(pool.clone())
        // .wrap(Logger::default())
        .service(
            web::resource("/{id}/{name}/index.html").to(index))
        )
        .bind("127.0.0.1:8080")?
        .run()
}
