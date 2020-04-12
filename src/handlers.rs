use crate::models::Status;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

pub async fn index2() -> impl Responder {
    HttpResponse::Ok().body("Hello world again!")
}

pub async fn status() -> impl Responder {
    HttpResponse::Ok().json(Status {
        status: "200".to_string()
    })
}