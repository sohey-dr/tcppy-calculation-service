use actix_web::{Responder, get};
use crate::adapter::presenter::decimal;
use crate::application::decimal::multiplication::usecase;

#[get("/multiplication")]
pub async fn multiplication() -> impl Responder {
    let output = usecase::exec();

    return decimal::multiplication_json_presenter::exec(output);
}
