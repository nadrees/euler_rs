use std::mem::{replace, swap};

use num_bigint::BigUint;
use num_traits::One;

/// a fibonacci sequence generator
pub struct Fibonacci {
    a: BigUint,
    b: BigUint,
}

impl Fibonacci {
    /// Starts a new sequence, returning 1, 2 as the first two elements
    pub fn new() -> Fibonacci {
        Self {
            a: One::one(),
            b: BigUint::new(vec![2]),
        }
    }

    /// Starts a new sequence, returning 1, 1 as the first two elements
    pub fn new_with_1_and_1() -> Fibonacci {
        Self {
            a: One::one(),
            b: One::one(),
        }
    }
}

impl Iterator for Fibonacci {
    type Item = BigUint;

    fn next(&mut self) -> Option<Self::Item> {
        let next = &self.a + &self.b;
        swap(&mut self.a, &mut self.b);
        Some(replace(&mut self.b, next))
    }
}
