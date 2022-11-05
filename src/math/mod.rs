mod proper_divisors;

use num_bigint::BigUint;

pub use proper_divisors::*;

/// Computes n!
pub fn factorial(n: u128) -> BigUint {
    let mut num: BigUint = BigUint::from(1u8);
    for i in 1u128..=n {
        num *= i;
    }
    return num;
}

/// Let d(n) be defined as the sum of proper divisors of n (numbers less than n which divide evenly into n).
///
/// If d(a) = b and d(b) = a, where a ≠ b, then a and b are an amicable pair and each of a and b are called amicable numbers.
///
/// Returns the amicable number b if it exists.
pub fn get_amicable_partner(a: &u128) -> Option<u128> {
    let b = proper_divisors(a).sum::<u128>();
    if b == *a {
        return None;
    }
    let b_divisors_sum = proper_divisors(&b).sum::<u128>();
    if b_divisors_sum == *a {
        return Some(b);
    } else {
        return None;
    }
}

#[cfg(test)]
mod tests {
    use super::get_amicable_partner;

    #[test]
    fn test_get_amicable_partner_1() {
        assert_eq!(None, get_amicable_partner(&1));
    }

    #[test]
    fn test_get_amicable_partner_220() {
        assert_eq!(Some(284), get_amicable_partner(&220));
    }
}
