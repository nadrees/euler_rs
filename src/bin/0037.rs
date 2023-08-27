use std::collections::HashSet;

use euler_rs::generators::Primes;

fn main() {
    let mut truncatable_primes = vec![];
    let mut primes = HashSet::new();
    for prime in Primes::new() {
        primes.insert(format!("{}", prime));
        if prime > 7 && is_truncatable_prime(prime, &primes) {
            truncatable_primes.push(prime);
            if truncatable_primes.len() == 11 {
                break;
            }
        }
    }
    println!("{:?}", truncatable_primes);
    let answer: u128 = truncatable_primes.into_iter().sum();
    print!("{}", answer);
}

fn is_truncatable_prime(prime: u128, primes: &HashSet<String>) -> bool {
    let prime = format!("{}", prime);
    let prime_len = prime.chars().count();

    for i in 0..prime_len {
        if !primes.contains(&prime[i..].to_string())
            || !primes.contains(&prime[0..prime_len - i].to_string())
        {
            return false;
        }
    }
    true
}
