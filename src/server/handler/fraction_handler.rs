use actix_web::{HttpResponse, Responder, get};
use crate::domain::model::fraction;
use crate::application::presenter::fraction;

#[get("/add")]
pub async fn add() -> impl Responder {
    let f1 = fraction::Fraction::new(1, 2);
    let f2 = fraction::Fraction::new(1, 2);
    let result = fraction::add(f1, f2);

    // 約分
    let reduced = result.reduce();

    if reduced.numerator() % reduced.denominator() == 0 {
        HttpResponse::Ok().body(format!("{}", reduced.numerator() / reduced.denominator()))
    } else {
        HttpResponse::Ok().body(format!("{}", reduced.to_string()))
    }
}
