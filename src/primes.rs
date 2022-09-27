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
    pub fn new(limit: u128) -> Self {
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
            if !self.bitarray.get(i) {
                self.previous_primes.push(i);
            }
        }
        self.bitarray = BitArray::new(min(self.offset + MAX_ITERATION_SIZE, self.limit));
        self.offset += MAX_ITERATION_SIZE;
        for prime in self.previous_primes.clone() {
            self.mark_multiples(prime);
        }
    }
}

impl Iterator for Primes {
    type Item = u128;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.index;
        if index >= self.limit {
            return None;
        } else if index < self.bitarray.len() {
            self.index += 1;
            if !self.bitarray.get(index) {
                self.mark_multiples(index);
                return Some(index);
            } else {
                return self.next();
            }
        } else {
            self.paginate();
            return self.next();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Primes;

    #[test]
    fn test_primes() {
        let p = Primes::new(10);
        let primes = p.collect::<Vec<_>>();
        assert_eq!(primes, vec![2, 3, 5, 7]);
    }
}
