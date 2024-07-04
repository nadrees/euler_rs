use super::Primes;

/// A number that is a sum of consecutive prime numbers. Returned by calls
/// to [`ConsecutivePrimeSum::next`].
#[derive(Debug, PartialEq, Eq)]
pub struct ConsecutivePrimeSumResult {
    /// The number that is a sum of consecutive prime numbers
    pub number: u128,
    /// The number of consecutive prime numbers that summed to `number`
    pub segments_length: usize,
}

/// An iterator that returns numbers that are sums of consecutive primes.
/// Ex:
/// 41 = 2 + 3 + 5 + 7 + 11 + 13
pub struct ConsecutivePrimeSum {
    below: u128,
    primes: Vec<u128>,
    current_number: u128,
    current_index: usize,
    current_window_size: usize,
}

impl ConsecutivePrimeSum {
    /// Returns a new iterator which will return [ConsecutivePrimeSumResult]s
    /// whos `number` is up to (but not including) `below`
    pub fn new(below: u128) -> Self {
        let primes = Primes::new_up_to(below).collect::<Vec<_>>();
        Self {
            below,
            primes,
            current_number: 0,
            current_index: 0,
            current_window_size: 2,
        }
    }

    /// Returns an iterator over the primes up to (but not including) `below`
    pub fn primes(&self) -> impl Iterator<Item = &u128> {
        self.primes.iter()
    }
}

impl Iterator for ConsecutivePrimeSum {
    type Item = ConsecutivePrimeSumResult;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index + self.current_window_size > self.primes.len() {
            if self.current_index == 0 {
                return None;
            }
            self.current_index = 0;
            self.current_window_size += 1;
            return self.next();
        }
        if self.current_index == 0 {
            self.current_number = self.primes
                [self.current_index..(self.current_index + self.current_window_size)]
                .iter()
                .sum();
        } else {
            self.current_number += self.primes[self.current_index + self.current_window_size - 1];
            self.current_number -= self.primes[self.current_index - 1];
        }
        if self.current_number >= self.below {
            if self.current_index == 0 {
                return None;
            }
            self.current_index = 0;
            self.current_window_size += 1;
            return self.next();
        }
        self.current_index += 1;
        Some(ConsecutivePrimeSumResult {
            number: self.current_number,
            segments_length: self.current_window_size,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consecutive_prime_sums_forward() {
        let mut cps = ConsecutivePrimeSum::new(11);
        assert_eq!(
            cps.next(),
            Some(ConsecutivePrimeSumResult {
                number: 5,
                segments_length: 2,
            })
        );
        assert_eq!(
            cps.next(),
            Some(ConsecutivePrimeSumResult {
                number: 8,
                segments_length: 2,
            })
        );
        assert_eq!(
            cps.next(),
            Some(ConsecutivePrimeSumResult {
                number: 10,
                segments_length: 3,
            })
        );
        assert_eq!(cps.next(), None);
    }
}
