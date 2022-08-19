use actix_web::{HttpResponse, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct HelloWorld {
    message: &'static str,
}

pub async fn index() -> impl Responder {
    HttpResponse::Ok().json(HelloWorld {
        message: "Hello world!",
    })
}
