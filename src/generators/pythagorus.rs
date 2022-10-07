/// A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,
/// a^2 + b^2 = c^2.
#[derive(Debug, PartialEq)]
pub struct PythagoreanTriplet {
    pub a: u128,
    pub b: u128,
    pub c: u128,
}

/// An iterator to return Pythagorean Triplets
pub struct PythagoreanTriplets {
    a: u128,
    b: u128,
    c: u128,
}

impl PythagoreanTriplets {
    /// Creates a new PythagoreanTriplets iterator
    pub fn new() -> Self {
        Self { a: 1, b: 1, c: 1 }
    }
}

impl Iterator for PythagoreanTriplets {
    type Item = PythagoreanTriplet;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.b == self.c {
                self.c += 1;
                self.a = 1;
                self.b = 1;
                continue;
            } else if self.a == self.b {
                self.b += 1;
                self.a = 1;
                continue;
            }
            let a = self.a;
            self.a += 1;
            if (a * a) + (self.b * self.b) == (self.c * self.c) {
                return Some(PythagoreanTriplet {
                    a,
                    b: self.b,
                    c: self.c,
                });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{PythagoreanTriplet, PythagoreanTriplets};

    #[test]
    fn test_pythagorean_triplets() {
        let triplet = PythagoreanTriplets::new().take(1).last().unwrap();
        assert_eq!(triplet, PythagoreanTriplet { a: 3, b: 4, c: 5 });
    }
}
