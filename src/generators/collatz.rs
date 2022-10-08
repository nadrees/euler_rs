/// A sequence of numbers generated starting a n and then
/// applying the following rules:
///
/// n → n/2 (n is even)
/// n → 3n + 1 (n is odd)
pub struct Collatz {
    n: u128,
}

impl Collatz {
    /// Starts a new Collatz sequence from n
    pub fn new(n: u128) -> Self {
        Self { n }
    }
}

impl Iterator for Collatz {
    type Item = u128;

    fn next(&mut self) -> Option<Self::Item> {
        if self.n == 0 {
            return None;
        }
        let n = self.n;
        if self.n == 1 {
            self.n = 0;
        } else if self.n % 2 == 0 {
            self.n = self.n / 2;
        } else {
            self.n = 3 * self.n + 1;
        }
        return Some(n);
    }
}

#[cfg(test)]
mod tests {
    use super::Collatz;

    #[test]
    fn test_collatz() {
        assert_eq!(
            vec![13, 40, 20, 10, 5, 16, 8, 4, 2, 1],
            Collatz::new(13).collect::<Vec<_>>()
        )
    }
}
