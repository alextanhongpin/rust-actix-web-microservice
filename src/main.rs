extern crate actix;
extern crate dotenv;
extern crate futures;
extern crate r2d2_mysql;
#[macro_use]
extern crate serde_derive;

// Exports Addr.
use actix::prelude::*;
use actix_web::{web, App, Error, HttpResponse, HttpServer};
use dotenv::dotenv;
use std::env;
use r2d2_mysql::mysql::{from_row};
use futures::Future;

mod db;
mod model;

// fn index(info: web::Path<(u32, String)>) -> impl Responder {
//     format!("hello {} id: {}", info.1, info.0)
// }

fn test(path: web::Path<String>, db: web::Data<db::Pool>) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move||{
        let mut conn = db.get().unwrap();
        // let posts: Vec<model::Post> = conn.prep_exec("SELECT * FROM post", ()).map(|result| {
        conn.prep_exec("SELECT * FROM post", ()).map(|result| {
            result.map(|x| x.unwrap()).map(|row| {
                let (id, title, body, published) = from_row(row);
                model::Post{
                    id,
                    title,
                    body,
                    published
                }
            }).collect::<Vec<model::Post>>()
        })
        // println!("got posts: {:?}", posts);
        // Ok(posts)
    }).then(|res| match res {
        Ok(posts) => Ok(HttpResponse::Ok().json(posts)),
        Err(_) => Ok(HttpResponse::InternalServerError().into())
    })

}

// pub struct AppState {
//     pub db: Addr<db::DbExecutor>
// }

fn main() -> std::io::Result<()> {
    // Grab the env vars.
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    // let sys = actix::System::new("Actix_Tutorial");
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

    // let addr: Addr<DbExecutor> = SyncArbiter::start(4, move|| db::DbExecutor(pool.clone()));
    HttpServer::new( move || {
        // App::with_state(AppState{db: addr.clone()})
        App::new()
            .data(pool.clone())
        // .wrap(Logger::default())
        // .service( web::resource("/base/{id}/{name}/index.html").to(index))
            .route( "/{name}", web::get().to_async(test))

    })
    .bind("127.0.0.1:8080")?
        .run()
    // sys.run();
}
