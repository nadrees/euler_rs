use std::{collections::HashSet, str::FromStr};

use euler_rs::generators::{DigitReplacement, DigitReplacementPattern, Primes};
use itertools::Itertools;

pub fn main() {
    let primes = Primes::new_up_to(1_000_000).collect::<HashSet<_>>();

    let mut max_length_found = 0;
    let mut primes_seen = HashSet::new();

    for p in primes.iter() {
        if primes_seen.contains(p) {
            continue;
        }
        let digit_replacement_pattern = DigitReplacementPattern::new(&format!("{}", p));
        for drp in digit_replacement_pattern {
            let prime_nums = DigitReplacement::new(&drp)
                .map(|n| <u128 as FromStr>::from_str(&n).unwrap())
                .filter(|n| primes.contains(n))
                .collect_vec();
            primes_seen.extend(prime_nums.clone());

            if prime_nums.len() > max_length_found {
                max_length_found = prime_nums.len();
                println!(
                    "{} prime family: {}",
                    max_length_found,
                    prime_nums.into_iter().sorted().join(", ")
                );
                if max_length_found == 8 {
                    return;
                }
            }
        }
    }
}
