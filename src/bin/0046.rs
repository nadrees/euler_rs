use euler_rs::{cached_iterator::CachedIterator, generators::Primes};

pub fn main() {
    let mut primes = CachedIterator::new(Primes::new());
    for n in 3..=u128::MAX {
        if n % 2 == 0 {
            continue;
        }
        let primes = primes.iter().take_while(|p| *p <= n).collect::<Vec<_>>();
        if primes.contains(&n) {
            continue;
        }
        if !can_decompose(&n, &primes) {
            println!("{}", n);
            break;
        }
    }
}

fn can_decompose(num: &u128, primes: &Vec<u128>) -> bool {
    for prime in primes {
        let remainder = num - prime;
        if remainder <= 1 {
            continue;
        }
        if can_decompose_to_twice_square(&remainder) {
            return true;
        }
    }
    return false;
}

fn can_decompose_to_twice_square(remainder: &u128) -> bool {
    for i in 1..=(remainder / 2) {
        let possible_decomposition = 2 * (i * i);
        if possible_decomposition == *remainder {
            return true;
        }
    }
    return false;
}
