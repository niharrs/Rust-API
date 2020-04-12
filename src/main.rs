mod models;
mod config;
mod handlers;
mod db;

use crate::models::Status;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::env;
use dotenv::dotenv;
use tokio_postgres::NoTls;
use crate::handlers::*;

pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

pub async fn index2() -> impl Responder {
    HttpResponse::Ok().body("Hello world again!")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    // let HOST = env::var("HOST").expect("Host not set");
    // let PORT = env::var("PORT").expect("Port not set");

    let config = crate::config::Config::from_env().unwrap();

    let pool = config.pg.create_pool(NoTls).unwrap();

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .route("/", web::get().to(index))
            .route("/again", web::get().to(index2))
            .route("/status", web::get().to(status))
            .route("/todos {_:/?}", web::get().to(get_todos))
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}