use std::collections::HashSet;

use euler_rs::generators::Primes;
use itertools::Itertools;

fn main() {
    let primes = Primes::new_up_to(1000000).collect::<HashSet<_>>();
    let answer = primes
        .iter()
        .filter(|prime| rotations_are_all_prime(prime, &primes))
        .count();
    print!("{}", answer);
}

fn rotations_are_all_prime(prime: &u128, primes: &HashSet<u128>) -> bool {
    let mut chars = prime.to_string().chars().collect_vec();
    for _i in 1..chars.len() {
        chars.rotate_left(1);
        let new_num: u128 = String::from_iter(&chars).parse().unwrap();
        if !primes.contains(&new_num) {
            return false;
        }
    }
    true
}
