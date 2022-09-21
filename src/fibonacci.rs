/// a fibonacci sequence generator
pub struct Fibonacci {
    a: u64,
    b: u64,
}

impl Fibonacci {
    /// Starts a new sequence
    pub fn new() -> Fibonacci {
        Self { a: 0, b: 0 }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.a == 0 {
            self.a = 1;
            return Some(self.a);
        } else if self.b == 0 {
            self.b = 2;
            return Some(self.b);
        } else {
            let next = self.a + self.b;
            self.a = self.b;
            self.b = next;
            return Some(next);
        }
    }
}
