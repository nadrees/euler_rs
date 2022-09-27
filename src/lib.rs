pub mod bitarray;
pub mod cached_iterator;
pub mod fibonacci;
pub mod prime_factors;
pub mod primes;

pub fn is_palindrome(s: &str) -> bool {
    let chars = s.chars().collect::<Vec<_>>();
    for i in 0..chars.len() / 2 {
        if chars[i] != chars[chars.len() - i - 1] {
            return false;
        }
    }
    return true;
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
