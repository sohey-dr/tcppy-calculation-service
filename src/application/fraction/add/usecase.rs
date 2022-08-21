use crate::domain::model::fraction::fraction;
use crate::application::fraction::add::usecase_output::FractionAddUsecaseOutput;

pub fn exec() -> FractionAddUsecaseOutput {
  let f1 = fraction::new_random_fraction();
  let f2 = fraction::new_random_fraction();
  let added = fraction::add(f1, f2);

  let f1_reduced = if !f1.eq(&f1.reduce()) {
    f1.reduce()
  } else {
    f1
  };
  let f2_reduced = if !f2.eq(&f2.reduce()) {
    f2.reduce()
  } else {
    f2
  };

  let mut calculation_process = fraction::reduce_denominators(vec![f1_reduced, f2_reduced]);

  let reduced = added.reduce();

  let result = if reduced.denominator() == 1 {
    calculation_process.push(reduced.to_string());
    format!("{}", reduced.numerator())
  } else {
    reduced.to_string()
  };

  return FractionAddUsecaseOutput {
    calculation_formula: vec![f1.to_string(), "+".to_string(), f2.to_string()],
    result: result,
    calculation_process: calculation_process,
  };
}
