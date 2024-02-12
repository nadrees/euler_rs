use std::ops::AddAssign;

use num_traits::{One, Unsigned};

use crate::{Three, Two};

pub trait PentagonalNumber: Unsigned {}
impl<N> PentagonalNumber for N where N: Unsigned {}

pub struct PentagonalNumberGenerator<N: PentagonalNumber> {
    n: N,
}

impl<N> Iterator for PentagonalNumberGenerator<N>
where
    N: PentagonalNumber + Two + Three + AddAssign + Copy,
{
    type Item = N;

    fn next(&mut self) -> Option<Self::Item> {
        let three: N = Three::three();
        let answer = self.n * (three * self.n - One::one()) / Two::two();
        self.n += One::one();
        return Some(answer);
    }
}

impl<N: PentagonalNumber> PentagonalNumberGenerator<N> {
    pub fn new() -> Self {
        PentagonalNumberGenerator { n: One::one() }
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::PentagonalNumberGenerator;

    #[test]
    pub fn test_pentagonal_numbers() {
        let pentagonal_numbers: Vec<u32> = PentagonalNumberGenerator::new().take(8).collect_vec();
        assert_eq!(vec![1, 5, 12, 22, 35, 51, 70, 92], pentagonal_numbers);
    }
}
