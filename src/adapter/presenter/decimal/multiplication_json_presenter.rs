use actix_web::{HttpResponse, Responder};
use serde::Serialize;

use crate::application::decimal::multiplication::usecase_output::DecimalMultiplicationUsecaseOutput;

#[derive(Serialize)]
struct DecimalMultiplicationJsonResponse {
  expression: String,
  result: String,
}

pub fn exec(output: DecimalMultiplicationUsecaseOutput) -> impl Responder {
  return HttpResponse::Ok().json(DecimalMultiplicationJsonResponse {
    expression: output.expression,
    result: output.result,
  });
}
