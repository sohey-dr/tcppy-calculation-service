use actix_web::{HttpResponse, Responder, get};
use crate::domain::model::fraction;
use crate::adpter::presenter::fraction;

struct FractionAddUsecaseOutput {
  calculation_formula: Vec<String>,
  result: String,
  calculation_process: Vec<FractionAddCalculationProcess>,
}

struct FractionAddCalculationProcess {
  calculation_formula: String,
  result: String,
  calculation_process: String,
}

pub fn exec() -> FractionAddUsecaseOutput {
    let f1 = fraction::Fraction::new(1, 2);
    let f2 = fraction::Fraction::new(1, 2);
    let result = fraction::add(f1, f2);

    // 約分
    let reduced = result.reduce();

    return FractionAddUsecaseOutput
}
