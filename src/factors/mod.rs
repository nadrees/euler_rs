mod prime_factors;

use std::ops::{AddAssign, Div, Rem};

use num_traits::{One, Zero};

pub use prime_factors::PrimeFactors;

pub trait FactorType:
    PartialOrd + One + AddAssign + Zero + Rem<Output = Self> + Div<Output = Self> + Copy
{
}

/// An iterator over the factors of a given number.
/// The factors may be returned in any order
pub struct Factors<'a, T: FactorType> {
    n: &'a T,
    index: T,
    // when we find a factor, this will hold the other factor
    // until we can return it from the iterator
    other_factor: Option<T>,
    // this is the maximum position we need to search until.
    // this gets decreased each time we find a factor because
    // we will return the factor and its counterpart, so there
    // is no need to search past its counterpart
    limit: T,
}

impl<'a, T: FactorType> Factors<'a, T> {
    /// Start a new iterator over the factors of n
    pub fn new(n: &'a T) -> Self {
        Self {
            n,
            index: One::one(),
            other_factor: None,
            limit: *n,
        }
    }
}

impl<'a, T: FactorType> Iterator for Factors<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.other_factor.is_some() {
            let other_factor = self.other_factor;
            self.other_factor = None;
            return other_factor;
        }
        while self.index < self.limit {
            let index = self.index;
            self.index += One::one();
            if *self.n % index == Zero::zero() {
                self.limit = *self.n / index;
                if self.limit != index {
                    self.other_factor = Some(self.limit);
                }
                return Some(index);
            }
        }
        return None;
    }
}

macro_rules! factor_type_impl {
    ($T:ty) => {
        impl FactorType for $T {}
    };
}

factor_type_impl!(u8);
factor_type_impl!(u16);
factor_type_impl!(u32);
factor_type_impl!(u64);
factor_type_impl!(u128);
factor_type_impl!(usize);

#[cfg(test)]
mod tests {
    use super::Factors;

    #[test]
    fn test_factors() {
        let mut factors = Factors::new(&28u8).collect::<Vec<_>>();
        factors.sort();
        assert_eq!(vec![1, 2, 4, 7, 14, 28], factors);
    }
}
