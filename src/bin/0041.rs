use std::collections::HashSet;

use euler_rs::generators::{pandigital_numbers_of_length, Primes};

fn main() {
    let primes = Primes::new_up_to(7654321).collect::<HashSet<_>>();
    let mut n = 7;
    while n > 0 {
        println!("Searching for pandigital primes of length {}", n);
        let largest_pandigital_primes = pandigital_numbers_of_length(n)
            .filter(|num| primes.contains(num))
            .max();
        match largest_pandigital_primes {
            Some(largest_pandigital_primes) => {
                print!("{}", largest_pandigital_primes);
                break;
            }
            None => n -= 1,
        }
    }
}
