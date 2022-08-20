pub struct fraction {
    numerator: i32,
    denominator: i32,
}

impl fraction {
    pub fn new(numerator: i32, denominator: i32) -> fraction {
        fraction {
            numerator,
            denominator,
        }
    }

    pub fn numerator(&self) -> i32 {
        self.numerator
    }

    pub fn denominator(&self) -> i32 {
        self.denominator
    }

    pub fn to_string(&self) -> String {
        format!("{}/{}", self.numerator, self.denominator)
    }
}
