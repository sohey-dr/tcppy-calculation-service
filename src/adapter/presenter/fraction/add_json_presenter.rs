use actix_web::{HttpResponse, Responder};
use serde::Serialize;

use crate::application::fraction::add::usecase_output::FractionAdditionUsecaseOutput;

#[derive(Serialize)]
struct FractionAdditionJsonResponse {
  calculation_formula: String,
  initial_reduced_fractions: String,
  result: String,
  calculation_process: Vec<String>,
}

pub fn exec(output: FractionAdditionUsecaseOutput) -> impl Responder {
  let calculation_formula = output.calculation_formula.join(" ");

  let result = output.result;

  return HttpResponse::Ok().json(FractionAdditionJsonResponse {
    calculation_formula: calculation_formula,
    initial_reduced_fractions: output.initial_reduced_fractions.join(" "),
    result: result,
    calculation_process: output.calculation_process,
  });
}
