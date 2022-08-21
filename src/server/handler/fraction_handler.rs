use actix_web::{HttpResponse, Responder, get};
use crate::domain::model::fraction;

#[get("/add")]
pub async fn add() -> impl Responder {
    let f1 = fraction::Fraction::new(1, 2);
    let f2 = fraction::Fraction::new(1, 2);
    let result = fraction::add(f1, f2);

    HttpResponse::Ok().body(result.to_string())
}
