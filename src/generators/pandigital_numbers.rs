use itertools::Itertools;

pub fn pandigital_numbers_of_length(length: usize) -> impl Iterator<Item = u128> {
    assert!(
        length > 0 && length <= 10,
        "0 < length <= 10, {} was provided",
        length
    );
    let digits = match length {
        10 => Vec::from_iter(0..10),
        _ => Vec::from_iter(1..=length),
    };
    digits.into_iter().permutations(length).map(|permutation| {
        permutation
            .into_iter()
            .fold(0u128, |acc, next| acc * 10 + (next as u128))
    })
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use super::*;

    #[test]
    fn test_pandigital_numbers_of_length() {
        let nums = pandigital_numbers_of_length(2).collect::<HashSet<_>>();
        assert_eq!(nums, HashSet::from_iter(vec![12, 21]));
    }

    #[test]
    fn test_pandigital_numbers_of_length_3() {
        let nums = pandigital_numbers_of_length(3).collect::<HashSet<_>>();
        assert_eq!(nums, HashSet::from_iter(vec![123, 132, 213, 231, 312, 321]));
    }

    #[test]
    fn test_pandigital_numbers_of_length_are_pandigital() {
        for num in pandigital_numbers_of_length(10) {
            let mut digits = HashMap::new();
            let mut n = num;
            while n > 0 {
                let digit = n % 10;
                if let Some(count) = digits.get(&digit) {
                    digits.insert(digit, count + 1);
                } else {
                    digits.insert(digit, 1usize);
                }
                n /= 10;
            }
            for k in digits.keys() {
                assert_eq!(
                    digits.get(k),
                    Some(&1usize),
                    "{} had zero or multiple {}",
                    num,
                    k
                );
            }
        }
    }
}
