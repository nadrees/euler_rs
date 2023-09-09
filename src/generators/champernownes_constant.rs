/// Champernowne's constant is an irrational number
/// created by concatenating integers together:
/// 0.1234567891011121314151617..

pub struct ChampernownesConstant {
    next: u128,
    current_digits: Vec<u8>,
    current_digits_index: usize,
}

impl ChampernownesConstant {
    pub fn new() -> Self {
        let mut s = ChampernownesConstant {
            next: 0,
            current_digits: vec![],
            current_digits_index: 0,
        };
        s.increment();
        s
    }

    fn increment(&mut self) {
        self.next += 1;
        let mut digits = vec![];
        let mut num = self.next;
        while num > 0 {
            digits.push((num % 10) as u8);
            num /= 10;
        }
        digits.reverse();
        self.current_digits = digits;
        self.current_digits_index = 0;
    }
}

impl Iterator for ChampernownesConstant {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_digits_index >= self.current_digits.len() {
            self.increment();
        }
        let answer = self.current_digits[self.current_digits_index];
        self.current_digits_index += 1;
        Some(answer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_champernownes_constant() {
        let generator = ChampernownesConstant::new();
        let digits = generator.take(10).collect::<Vec<_>>();
        assert_eq!(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 1], digits);
    }
}
