mod prime_factors;

pub use prime_factors::PrimeFactors;

/// An iterator over the factors of a given number.
/// The factors may be returned in any order
pub struct Factors<'a> {
    n: &'a u128,
    index: u128,
    // when we find a factor, this will hold the other factor
    // until we can return it from the iterator
    other_factor: Option<u128>,
    // this is the maximum position we need to search until.
    // this gets decreased each time we find a factor because
    // we will return the factor and its counterpart, so there
    // is no need to search past its counterpart
    limit: u128,
}

impl<'a> Factors<'a> {
    /// Start a new iterator over the factors of n
    pub fn new(n: &'a u128) -> Self {
        Self {
            n,
            index: 1,
            other_factor: None,
            limit: *n,
        }
    }
}

impl<'a> Iterator for Factors<'a> {
    type Item = u128;

    fn next(&mut self) -> Option<Self::Item> {
        if self.other_factor.is_some() {
            let other_factor = self.other_factor;
            self.other_factor = None;
            return other_factor;
        }
        while self.index < self.limit {
            let index = self.index;
            self.index += 1;
            if self.n % index == 0 {
                self.limit = self.n / index;
                if self.limit != index {
                    self.other_factor = Some(self.limit);
                }
                return Some(index);
            }
        }
        return None;
    }
}

#[cfg(test)]
mod tests {
    use super::Factors;

    #[test]
    fn test_factors() {
        let mut factors = Factors::new(&28).collect::<Vec<_>>();
        factors.sort();
        assert_eq!(vec![1, 2, 4, 7, 14, 28], factors);
    }
}
