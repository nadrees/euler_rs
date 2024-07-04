use std::collections::HashSet;

use euler_rs::generators::ConsecutivePrimeSum;

pub fn main() {
    let cps: ConsecutivePrimeSum = ConsecutivePrimeSum::new(1_000_000);
    let primes = cps.primes().cloned().collect::<HashSet<_>>();

    let longest_prime = cps
        .into_iter()
        .filter(|n| primes.contains(&n.number))
        .max_by_key(|a| a.segments_length)
        .unwrap();
    println!("{}", longest_prime.number);
}
