use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

use itertools::Itertools;
use num_traits::Unsigned;

pub mod bigint;
pub mod bitarray;
pub mod cached_iterator;
pub mod factors;
pub mod generators;
pub mod grid;
pub mod math;
pub mod triangle;

pub fn is_prime<N, I>(num: N, primes_iter: I) -> bool
where
    N: Unsigned,
    I: Iterator<Item = u128>,
    u128: From<N>,
{
    let converted_n: u128 = N::into(num);
    primes_iter
        .take_while(|p| *p <= converted_n)
        .contains(&converted_n)
}

pub fn is_palindrome(s: &str) -> bool {
    let chars = s.chars().collect::<Vec<_>>();
    for i in 0..chars.len() / 2 {
        if chars[i] != chars[chars.len() - i - 1] {
            return false;
        }
    }
    return true;
}

/// Reads the lines of a file and returns them as an iterator
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use crate::is_palindrome;

    #[test]
    fn test_is_palindrome() {
        assert_eq!(is_palindrome(&"bob"), true);
        assert_eq!(is_palindrome(&"test"), false);
    }
}
