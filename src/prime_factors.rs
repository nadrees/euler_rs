use crate::{cached_iterator::CachedIterator, primes::Primes};

/// An iterator the returs the prime factors of n. It may return
/// a single factor more than once.
pub struct PrimeFactorIterator {
    n: u128,
    iter: CachedIterator<Primes, u128>,
}

impl PrimeFactorIterator {
    /// Creates a new iterator for n.
    pub fn new(n: u128) -> Self {
        Self {
            n,
            iter: CachedIterator::new(Primes::new(n / 2)),
        }
    }
}

impl Iterator for PrimeFactorIterator {
    type Item = <Primes as Iterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self
            .iter
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
    use crate::prime_factors::PrimeFactorIterator;

    #[test]
    fn test_prime_factors() {
        let prime_factors = PrimeFactorIterator::new(13195).collect::<Vec<_>>();
        assert_eq!(prime_factors, vec![5, 7, 13, 29]);
    }
}
