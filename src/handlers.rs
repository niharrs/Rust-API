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