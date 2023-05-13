use crate::bitarray::BitArray;
use std::cmp::min;

const MAX_ITERATION_SIZE: u128 = 128 * 100;

/// A prime number iterator
pub struct Primes {
    bitarray: BitArray,
    index: u128,
    offset: u128,
    limit: u128,
    previous_primes: Vec<u128>,
}

impl Primes {
    /// creates a new Prime number generator that can generate primes to limit - 1
    pub fn new() -> Self {
        Self::new_up_to(u128::MAX)
    }

    pub fn new_up_to(limit: u128) -> Self {
        let bitarray = BitArray::new(min(limit, MAX_ITERATION_SIZE));
        Self {
            bitarray,
            limit,
            index: 2,
            offset: 0,
            previous_primes: vec![],
        }
    }

    /// true means it is a duplicate, false it is not (and therefore a prime)
    fn mark_multiples(&mut self, p: u128) -> () {
        let mut idx = (p - self.offset) + p;
        while idx < self.bitarray.len() {
            self.bitarray.set(idx, true);
            idx += p;
        }
    }

    /// reset the bitarray to all false, and iteratively marks all multiples of
    /// previously found primes as true
    fn paginate(&mut self) -> () {
        for i in 0..self.bitarray.len() {
            let prime = i + self.offset;
            if prime > 1 && !self.bitarray.get(i) {
                self.previous_primes.push(prime);
            }
        }
        self.bitarray = BitArray::new(min(self.offset + MAX_ITERATION_SIZE, self.limit));
        self.offset += MAX_ITERATION_SIZE;
        for prime in self.previous_primes.clone() {
            let mut idx = prime;
            while idx < self.offset {
                idx += prime;
            }
            idx = idx - self.offset;
            while idx < self.bitarray.len() {
                self.bitarray.set(idx, true);
                idx += prime;
            }
        }
        self.index = 0;
    }
}

impl Iterator for Primes {
    type Item = u128;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            while self.index < self.bitarray.len() {
                let num = self.index + self.offset;
                if num >= self.limit {
                    return None;
                }
                let index = self.index;
                self.index += 1;
                if !self.bitarray.get(index) {
                    self.mark_multiples(num);
                    return Some(num);
                }
            }
            self.paginate();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Primes;

    #[test]
    fn test_primes() {
        let p = Primes::new_up_to(10);
        let primes = p.collect::<Vec<_>>();
        assert_eq!(primes, vec![2, 3, 5, 7]);
    }

    #[test]
    fn test_generates_2000_primes() {
        assert_eq!(
            Primes::new_up_to(u128::MAX).take(2000).last().unwrap(),
            17389
        );
    }

    #[test]
    fn test_generates_10001_prime() {
        assert_eq!(
            Primes::new_up_to(u128::MAX).take(10001).last().unwrap(),
            104743
        );
    }
}
