extern crate rand;
use rand::Rng;

use crate::application::decimal::multiplication::usecase_output::DecimalMultiplicationUsecaseOutput;

// TODO: 計算に使用する数字が2つのみになっているので、3つ以上に対応できるようにする
pub fn exec() -> DecimalMultiplicationUsecaseOutput {
  let mut rng = rand::thread_rng();
  let expression_nums = vec![rng.gen_range(0.0, 300.0), rng.gen_range(0.0, 300.0)];
  let expression = expression_nums[0].to_string() + " × " + &expression_nums[1].to_string();

  let result = expression_nums[0] * expression_nums[1];

  return DecimalMultiplicationUsecaseOutput {
    expression: expression,
    result: result.to_string(),
  };
}
