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
