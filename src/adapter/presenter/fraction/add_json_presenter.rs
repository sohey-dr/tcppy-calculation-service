use actix_web::{HttpResponse, Responder};
use serde::Serialize;

use crate::application::fraction::add::usecase_output::FractionAddUsecaseOutput;

#[derive(Serialize)]
struct FractionAddJsonResponse {
  calculation_formula: String,
  result: String,
  calculation_process: Vec<String>,
}

pub fn exec(output: FractionAddUsecaseOutput) -> impl Responder {
  let calculation_formula = output.calculation_formula.join(" ");

  let result = output.result;

  return HttpResponse::Ok().json(FractionAddJsonResponse {
    calculation_formula: calculation_formula,
    result: result,
    calculation_process: output.calculation_process,
  });
}
