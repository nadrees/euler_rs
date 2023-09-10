use itertools::Itertools;

pub fn pandigital_numbers_of_length(length: usize) -> impl Iterator<Item = u128> {
    let digits = Vec::from_iter(1..=length);
    digits.into_iter().permutations(length).map(|permutation| {
        permutation
            .into_iter()
            .fold(0u128, |acc, next| acc * 10 + (next as u128))
    })
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn test_pandigital_numbers_of_length() {
        let nums = pandigital_numbers_of_length(2).collect::<HashSet<_>>();
        assert_eq!(nums, HashSet::from_iter(vec![12, 21]));
    }
}
