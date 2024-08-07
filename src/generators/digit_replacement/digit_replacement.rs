/// An iterator that returns all numbers that are a result of replacing
/// *s in the provided pattern with sequential [0-9] digits. All *s are
/// replaced with the same digit at any given time.
///
/// Example:
/// 56**3 would yield
/// * 56003
/// * 56113
/// * 56223
///
/// ...
///
/// * 56993
pub struct DigitReplacement<'a> {
    current_replacement: u8,
    pattern: &'a str,
}

impl<'a> DigitReplacement<'a> {
    /// Creates a new [DigitReplacement] iterator with the provided `pattern`
    pub fn new(pattern: &'a str) -> Self {
        Self {
            pattern,
            current_replacement: 1,
        }
    }
}

impl<'a> Iterator for DigitReplacement<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_replacement == 10 {
            return None;
        }
        let result = self
            .pattern
            .replace("*", &self.current_replacement.to_string());
        self.current_replacement += 1;
        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::DigitReplacement;

    #[test]
    fn test_digit_pattern_replacement() {
        let r = DigitReplacement::new("1*");
        assert_eq!(
            r.into_iter().collect_vec(),
            vec!["11", "12", "13", "14", "15", "16", "17", "18", "19"]
        );
    }
}
