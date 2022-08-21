use actix_web::{HttpResponse, Responder, get};

#[get("/add")]
pub async fn add() -> impl Responder {
    HttpResponse::Ok().json("Hello world!")
}
