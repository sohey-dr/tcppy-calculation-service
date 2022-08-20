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
}

impl PartialEq for Fraction {
    fn eq(&self, other: &Fraction) -> bool {
        self.numerator == other.numerator && self.denominator == other.denominator
    }
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
