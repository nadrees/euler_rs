/// Returns the index of the character if it's an
/// english character.
///
/// Example:
/// ```
/// use euler_rs::alpha::english_char_indexed_value;
///
/// assert_eq!(english_char_indexed_value(&'A'), 1);
/// assert_eq!(english_char_indexed_value(&'a'), 1);
///
/// let result = std::panic::catch_unwind(|| english_char_indexed_value(&'\u{306}'));
/// assert!(result.is_err());
/// ```
pub fn english_char_indexed_value(c: &char) -> u32 {
    let ans = if *c >= 'a' && *c <= 'z' {
        (*c as u32) - ('a' as u32)
    } else if *c >= 'A' && *c <= 'Z' {
        (*c as u32) - ('A' as u32)
    } else {
        panic!("{} is not a valid english character", c)
    };
    ans + 1
}

/// Returns the sum of the indexes of the characters
/// of the string. See english_char_indexed_value for
/// details.
///
/// Example:
/// ```
/// use euler_rs::alpha::english_str_indexed_value;
///
/// assert_eq!(english_str_indexed_value(&"SKY"), 55);
/// ```
pub fn english_str_indexed_value(s: &str) -> u32 {
    s.chars().map(|c| english_char_indexed_value(&c)).sum()
}
