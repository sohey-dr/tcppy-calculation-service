use rand::prelude::*;
use num::integer;

#[derive (Clone, Copy, Debug)]
pub struct Fraction {
    numerator: i32,
    denominator: i32,
}

impl Fraction {
    pub fn new(numerator: i32, denominator: i32) -> Fraction {
        Fraction {
            numerator,
            denominator,
        }
    }

    pub fn numerator(&self) -> i32 {
        self.numerator.clone()
    }

    pub fn denominator(&self) -> i32 {
        self.denominator.clone()
    }

    pub fn to_string(&self) -> String {
        format!("{}/{}", self.numerator, self.denominator)
    }

    pub fn reduce(&self) -> Fraction {
        let mut numerator = self.numerator.clone();
        let mut denominator = self.denominator.clone();
        let gcd = self.gcd(numerator, denominator);
        numerator /= gcd;
        denominator /= gcd;
        Fraction::new(numerator, denominator)
    }

    fn gcd(&self, a: i32, b: i32) -> i32 {
        if b == 0 {
            a
        } else {
            self.gcd(b, a % b)
        }
    }
}

impl PartialEq for Fraction {
    fn eq(&self, other: &Fraction) -> bool {
        self.numerator == other.numerator && self.denominator == other.denominator
    }
}

pub fn addition(f1: Fraction, f2: Fraction) -> Fraction {
    let numerator = f1.numerator() * f2.denominator() + f2.numerator() * f1.denominator();
    let denominator = f1.denominator() * f2.denominator();

		// TODO: 通分する必要がなければしないようにする
    Fraction::new(numerator, denominator)
}

pub fn new_random_fraction() -> Fraction {
    let numerator = rand::thread_rng().gen_range(1, 10);
    let denominator = rand::thread_rng().gen_range(1, 10);
    Fraction::new(numerator, denominator)
}

pub fn reduce_denominators(fractions: Vec<Fraction>) -> Vec<String> {
    let mut lcm = integer::lcm(fractions[0].denominator(), fractions[1].denominator());
    for fraction in &fractions {
        lcm = integer::lcm(lcm, fraction.denominator());
    }

    fractions.iter().map(|f| {
        let numerator = f.numerator() * lcm / f.denominator();
        let denominator = f.denominator() * lcm / f.denominator();
        format!("{}/{}", numerator, denominator)
    }).collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fraction_new() {
      let fraction = Fraction::new(1, 2);
      assert_eq!(fraction.numerator(), 1);
      assert_eq!(fraction.denominator(), 2);
    }

    #[test]
    fn test_fraction_to_string() {
      let fraction = Fraction::new(1, 2);
      assert_eq!(fraction.to_string(), "1/2");
    }

    #[test]
    fn test_fraction_reduce() {
      let fraction = Fraction::new(2, 4);
      assert_eq!(fraction.reduce().to_string(), "1/2");
    }

    #[test]
    fn test_fraction_gcd() {
      let fraction = Fraction::new(2, 4);
      assert_eq!(fraction.gcd(2, 4), 2);
    }

    #[test]
    fn test_fraction_addition() {
      let f1 = Fraction::new(1, 2);
      let f2 = Fraction::new(1, 2);

			// TODO: 通分する必要がなければしないようにする
      assert_eq!(addition(f1, f2).to_string(), "4/4");
    }

    #[test]
    fn test_fraction_reduce_denominators() {
      let fractions = vec![Fraction::new(1, 2), Fraction::new(1, 3)];
      assert_eq!(reduce_denominators(fractions), vec!["3/6", "2/6"]);
    }

    #[test]
    fn test_fraction_new_random_fraction() {
      let fraction = new_random_fraction();

      assert!(fraction.numerator() > 0);
      assert!(fraction.denominator() > 0);
    }
}
