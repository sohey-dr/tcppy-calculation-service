use crate::domain::model::fraction::fraction;
use crate::application::fraction::add::usecase_output::FractionAddUsecaseOutput;

pub fn exec() -> FractionAddUsecaseOutput {
  let f1 = fraction::new_random_fraction();
  let f2 = fraction::new_random_fraction();
  let added = fraction::add(f1, f2);

  let reduced = added.reduce();

  let result = if reduced.denominator() == 1 {
    format!("{}", reduced.numerator())
  } else {
    reduced.to_string()
  };

  return FractionAddUsecaseOutput {
    calculation_formula: vec![f1.to_string(), "+".to_string(), f2.to_string()],
    result: result,
    calculation_process: vec!["1/2".to_string()],
  };
}
