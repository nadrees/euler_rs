use std::ops::AddAssign;

use num_traits::{One, Unsigned};

use crate::Two;

pub trait HexagonalNumber: Unsigned {}
impl<N> HexagonalNumber for N where N: Unsigned {}

pub struct HexagonalNumberGenerator<N: HexagonalNumber> {
    n: N,
}

impl<N: HexagonalNumber> HexagonalNumberGenerator<N> {
    pub fn new() -> Self {
        HexagonalNumberGenerator { n: One::one() }
    }
}

impl<N: HexagonalNumber> Iterator for HexagonalNumberGenerator<N>
where
    N: Two + AddAssign + Copy,
{
    type Item = N;

    fn next(&mut self) -> Option<Self::Item> {
        let two: N = Two::two();
        let answer = self.n * (two * self.n - One::one());
        self.n += One::one();
        return Some(answer);
    }
}

#[cfg(test)]
mod tests {
    use super::HexagonalNumberGenerator;

    #[test]
    pub fn test_hexagonal_numbers() {
        let mut hexagonal_numbers = HexagonalNumberGenerator::<u128>::new();
        assert_eq!(Some(40755), hexagonal_numbers.nth(142));
    }
}
