use std::collections::HashSet;

use euler_rs::math::{get_classification, Classification};

pub fn main() {
    let range = 1..=28123u16;

    let abundant_numbers: Vec<_> = range
        .clone()
        .filter(|i| get_classification(&(*i as u128)) == Classification::ABUNDANT)
        .collect();
    let mut non_abundant_sums: HashSet<u16> = HashSet::from_iter(range);

    for i in 0..abundant_numbers.len() {
        for j in i..abundant_numbers.len() {
            let sum = abundant_numbers[i] + abundant_numbers[j];
            if non_abundant_sums.contains(&sum) {
                non_abundant_sums.remove(&sum);
            }
        }
    }

    let answer: u128 = non_abundant_sums.iter().map(|s| *s as u128).sum();
    println!("{}", answer);
}
