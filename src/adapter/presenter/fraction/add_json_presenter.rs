use serde::Serialize;

#[derive(Serialize)]
struct FractionAddJsonResponse {
  calculation_formula: String,
  result: String,
  calculation_process: Vec<FractionAddCalculationProcess>,
}

#[derive(Serialize)]
struct FractionAddCalculationProcess {
  calculation_formula: String,
  result: String,
  calculation_process: String,
}
