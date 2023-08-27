mod fraction;
mod proper_divisors;

pub use fraction::*;
use num_bigint::BigUint;
pub use proper_divisors::*;
use std::{collections::HashSet, hash::Hash};

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

/// Computes the length of the cylce in the decimal representation of
/// the given fraction.
pub fn cycle_length<T>(f: Fraction<T>) -> Option<usize>
where
    T: FractionType + Eq + Hash,
{
    let mut digits_seen: Vec<T> = vec![];
    let mut decimals_iter = f.decimal_portion();
    while let Some(digit) = decimals_iter.next() {
        if !digits_seen.contains(&digit) {
            digits_seen.push(digit);
        } else {
            // walk backwards until we see the same decimal again
            let mut steps = 1;
            while let Some(previous_digit) = digits_seen.pop() {
                if previous_digit == digit {
                    return Some(steps);
                } else {
                    steps += 1;
                }
            }
            return None;
        }
    }
    return None;
}

pub fn is_pandigital(n: &str) -> bool {
    // we know that the largest 1 - 9 pandigital number is
    // 987,654,321. This means that any number with more than 9
    // digits cannot be pandigital.

    // The smallest 1 - 9 pandigital number is 123,456,789.
    // This means that any number with less than 9 digits cannot
    // be pandigital.
    let mut set = HashSet::new();
    let mut count = 0;
    for char in n.chars() {
        count += 1;
        if count > 9 {
            return false;
        } else if char.is_digit(10) && !set.contains(&char) {
            set.insert(char);
        } else {
            return false;
        }
    }
    return count == 9 && !set.contains(&'0');
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_amicable_partner_1() {
        assert_eq!(None, get_amicable_partner(&1));
    }

    #[test]
    fn test_get_amicable_partner_220() {
        assert_eq!(Some(284), get_amicable_partner(&220));
    }

    #[test]
    fn test_cycle_length() {
        assert_eq!(None, cycle_length(Fraction::new(1u8, 2)));
        assert_eq!(Some(1), cycle_length(Fraction::new(1u8, 3)));
        assert_eq!(Some(1), cycle_length(Fraction::new(1u8, 6)));
        assert_eq!(Some(6), cycle_length(Fraction::new(1u8, 7)));
    }
}
