mod models;
mod config;

use crate::models::Status;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::env;
use dotenv::dotenv;

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

async fn index2() -> impl Responder {
    HttpResponse::Ok().body("Hello world again!")
}

async fn status() -> impl Responder {
    HttpResponse::Ok().json(Status {
        status: "200".to_string()
    })
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    // let HOST = env::var("HOST").expect("Host not set");
    // let PORT = env::var("PORT").expect("Port not set");

    let config = crate::config::Config::from_env().unwrap();

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/again", web::get().to(index2))
            .route("/status", web::get().to(status))
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}