extern crate diesel;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_mysql;
extern crate r2d2_diesel;

use actix_web::{web, App, HttpServer, Responder};
use diesel::mysql::MysqlConnection;
use dotenv::dotenv;
use std::env;
use r2d2_diesel::{ConnectionManager};
use r2d2::{Pool};

pub type DBPool = Pool<ConnectionManager<MysqlConnection>>;
pub struct DbExecutor(pub DBPool);

fn index(info: web::Path<(u32, String)>) -> impl Responder {
    format!("hello {} id: {}", info.1, info.0)
}

fn init_pool(database_url: &str) -> DBPool {
    // Set default config for connection pool.
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    Pool::builder().build(manager).expect("failed to create pool")
}

fn main() -> std::io::Result<()> {
    // Grab the env vars.
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let pool = init_pool(&database_url);

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
