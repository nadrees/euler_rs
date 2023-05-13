use std::ops::AddAssign;

use num_traits::{One, Signed, Zero};

pub struct QuadraticFormula<T: Signed> {
    a: T,
    b: T,
    n: T,
}

impl<T: Signed> QuadraticFormula<T> {
    pub fn new(a: T, b: T) -> Self {
        QuadraticFormula {
            a,
            b,
            n: Zero::zero(),
        }
    }
}

impl<T: Signed + AddAssign + Copy + PartialOrd> Iterator for QuadraticFormula<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let next = (self.n * self.n) + (self.a * self.n) + self.b;
        if next <= One::one() {
            return None;
        }
        self.n += One::one();
        return Some(next);
    }
}
