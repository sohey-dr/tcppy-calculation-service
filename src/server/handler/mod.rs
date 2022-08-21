use actix_web::{HttpResponse, Responder, get};
use serde::Serialize;

#[derive(Serialize)]
struct HelloWorld {
    message: &'static str,
}

#[get("/")]
pub async fn index() -> impl Responder {
    HttpResponse::Ok().json(HelloWorld {
        message: "Hello world!",
    })
}
