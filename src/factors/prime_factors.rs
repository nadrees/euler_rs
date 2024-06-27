use std::sync::Mutex;

use crate::{cached_iterator::CachedIterator, generators::Primes};
use itertools::Itertools;
use lazy_static::lazy_static;

lazy_static! {
    static ref PRIMES: Mutex<CachedIterator<Primes, u128>> =
        Mutex::new(CachedIterator::new(Primes::new()));
}

/// An iterator the returs the prime factors of n. It may return
/// a single factor more than once.
pub struct PrimeFactors {
    n: u128,
}

impl PrimeFactors {
    /// Creates a new iterator for n.
    pub fn new(n: u128) -> Self {
        Self { n }
    }

    /// Returns the count of the distinct prime factors for n
    pub fn distinct_count(n: u128) -> usize {
        Self::new(n).unique().count()
    }
}

impl Iterator for PrimeFactors {
    type Item = <Primes as Iterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match PRIMES
            .lock()
            .unwrap()
            .iter()
            .take_while(|x| *x <= self.n)
            .find(|x| self.n % *x == 0)
        {
            None => None,
            Some(divisor) => {
                self.n /= divisor;
                Some(divisor)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PrimeFactors;

    #[test]
    fn test_prime_factors() {
        let prime_factors = PrimeFactors::new(13195).collect::<Vec<_>>();
        assert_eq!(prime_factors, vec![5, 7, 13, 29]);
    }

    #[test]
    fn test_distinct_count() {
        assert_eq!(PrimeFactors::distinct_count(14), 2); // 2 x 7
        assert_eq!(PrimeFactors::distinct_count(644), 3); // 2^2 x 7 x 23
    }
}
