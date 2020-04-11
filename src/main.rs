mod models;
mod config;
mod handlers;

use crate::models::Status;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::env;
use dotenv::dotenv;
use tokio_postgres::NoTls;

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
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}