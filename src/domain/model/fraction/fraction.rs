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
        let mut gcd = self.gcd(numerator, denominator);
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

pub fn add(f1: Fraction, f2: Fraction) -> Fraction {
    let numerator = f1.numerator() * f2.denominator() + f2.numerator() * f1.denominator();
    let denominator = f1.denominator() * f2.denominator();
    Fraction::new(numerator, denominator)
}

pub fn new_random_fraction() -> Fraction {
    let numerator = rand::thread_rng().gen_range(1, 10);
    let denominator = rand::thread_rng().gen_range(1, 10);
    Fraction::new(numerator, denominator)
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
}
