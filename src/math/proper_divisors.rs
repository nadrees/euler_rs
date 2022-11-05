use crate::factors::Factors;

/// Returns an iterator over the proper divisors of n.
/// Proper divisors are all factors a of n where a < n.
pub fn proper_divisors(n: &u128) -> impl Iterator<Item = u128> + '_ {
    Factors::new(n).filter(move |f| *f < *n)
}

#[derive(Debug, Eq, PartialEq)]
/// The classification of the number
pub enum Classification {
    /// A number is considered "deficient" if the sum of its proper divisors < num
    DEFICIENT = 0,
    /// A number is considered "perfect" if the sum of its proper divisors = num
    PERFECT = 1,
    /// A number is considered "abundant" if the sum of its proper divisors > num
    ABUNDANT = 2,
}

pub fn get_classification(n: &u128) -> Classification {
    let proper_divisor_sum = proper_divisors(n).sum::<u128>();
    if proper_divisor_sum < *n {
        Classification::DEFICIENT
    } else if proper_divisor_sum == *n {
        Classification::PERFECT
    } else {
        Classification::ABUNDANT
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_proper_divisors() {
        let mut divisors = proper_divisors(&220).collect::<Vec<_>>();
        divisors.sort();
        assert_eq!(divisors, vec![1, 2, 4, 5, 10, 11, 20, 22, 44, 55, 110])
    }

    #[test]
    fn test_get_classification() {
        assert_eq!(get_classification(&4), Classification::DEFICIENT);
        assert_eq!(get_classification(&12), Classification::ABUNDANT);
        assert_eq!(get_classification(&28), Classification::PERFECT);
    }
}
