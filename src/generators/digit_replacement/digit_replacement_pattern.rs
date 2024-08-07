use std::vec::IntoIter;

use itertools::{Combinations, Itertools};

/// An iterator that returns all digits replacements for all combinations
/// of patterns for a given string.
///
/// Example:
/// 56110 would generate the patterns
/// * *6110
/// * 5*110
/// * 56*10
///
/// ...
///
/// * 5611*
/// * **110
/// * \*6*10
///
/// ...
///
/// etc.
pub struct DigitReplacementPattern {
    pattern: Vec<char>,
    indexes: Vec<usize>,
    k: usize,
    indexes_iterator: Combinations<IntoIter<usize>>,
}

impl DigitReplacementPattern {
    pub fn new(pattern: &str) -> Self {
        let pattern = pattern.chars().collect_vec();
        let indexes = (0..(pattern.len())).collect_vec();
        let k = 1;
        let indexes_iterator = indexes.clone().into_iter().combinations(k);
        Self {
            indexes,
            pattern,
            k,
            indexes_iterator,
        }
    }
}

impl Iterator for DigitReplacementPattern {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.k == self.indexes.len() {
            return None;
        }
        if let Some(wildecard_indexes) = self.indexes_iterator.next() {
            let mut pattern = self.pattern.clone().to_owned();
            for wildcard_index in &wildecard_indexes {
                pattern[*wildcard_index] = '*';
            }
            return Some(pattern.into_iter().collect());
        }
        self.k += 1;
        self.indexes_iterator = self.indexes.clone().into_iter().combinations(self.k);
        return self.next();
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::DigitReplacementPattern;

    #[test]
    fn test_digit_replacement_pattern() {
        let drp = DigitReplacementPattern::new(&"1234");
        assert_eq!(
            drp.collect_vec(),
            vec![
                "*234", "1*34", "12*4", "123*", "**34", "*2*4", "*23*", "1**4", "1*3*", "12**",
                "***4", "**3*", "*2**", "1***"
            ]
        );
    }
}
