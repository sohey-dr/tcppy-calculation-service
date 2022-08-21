use actix_web::{HttpResponse, Responder, get};
use crate::adpter::presenter::fraction;
use crate::application::fraction::add::usecase;

#[get("/add")]
pub async fn add() -> impl Responder {
    let output = usecase::exec();

    return fraction::add_json_presenter::exec(output);
}
