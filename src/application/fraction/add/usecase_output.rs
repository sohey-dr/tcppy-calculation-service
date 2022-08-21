pub struct FractionAddUsecaseOutput {
  pub calculation_formula: Vec<String>,
  pub result: String,
  pub calculation_process: Vec<FractionAddCalculationProcess>,
}

pub struct FractionAddCalculationProcess {
  pub calculation_formula: String,
}
