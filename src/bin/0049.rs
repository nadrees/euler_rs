use std::collections::HashSet;

use euler_rs::{digits, generators::Primes};
use itertools::Itertools;

pub fn main() {
    let primes = Primes::new()
        .skip_while(|p| *p < 1000)
        .take_while(|p| *p < 10000)
        .collect::<HashSet<_>>();
    for prime in &primes {
        let permutations = digits(*prime)
            .permutations(4)
            .map(|permutation| {
                (permutation[0] as u128 * 1000)
                    + (permutation[1] as u128 * 100)
                    + (permutation[2] as u128 * 10)
                    + (permutation[3] as u128)
            })
            .sorted()
            .collect::<Vec<_>>();
        for combinations in permutations.into_iter().combinations(3) {
            // all elements must be prime
            if !primes.contains(&combinations[0])
                || !primes.contains(&combinations[1])
                || !primes.contains(&combinations[2])
            {
                continue;
            }
            // difference between elements must be the same
            if combinations[2] - combinations[1] != combinations[1] - combinations[0] {
                continue;
            }
            println!("{}", combinations.into_iter().join(""));
            return;
        }
    }
}
