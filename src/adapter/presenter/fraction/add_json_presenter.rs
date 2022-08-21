use actix_web::{HttpResponse, Responder};
use serde::Serialize;

use crate::domain::model::fraction;
use crate::application::fraction::add::usecase_output::FractionAddUsecaseOutput;

#[derive(Serialize)]
struct FractionAddJsonResponse {
  calculation_formula: String,
  result: String,
  calculation_process: Vec<FractionAddCalculationProcess>,
}

#[derive(Serialize)]
struct FractionAddCalculationProcess {
  process: String,
}

pub fn exec(output: FractionAddUsecaseOutput) -> impl Responder {
  let calculation_formula = output.calculation_formula.join(" ");

  let result = output.result;

  let calculation_process = vec![
    FractionAddCalculationProcess {
      process: "1/2".to_string(),
    }
  ];

  return HttpResponse::Ok().json(FractionAddJsonResponse {
    calculation_formula: calculation_formula,
    result: result,
    calculation_process: calculation_process,
  });
}
