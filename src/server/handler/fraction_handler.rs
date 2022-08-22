use actix_web::{Responder, get};
use crate::adapter::presenter::fraction;
use crate::application::fraction::addition::usecase;

#[get("/addition")]
pub async fn addition() -> impl Responder {
    let output = usecase::exec();

    return fraction::addition_json_presenter::exec(output);
}
