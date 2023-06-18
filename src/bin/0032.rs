use std::collections::HashSet;

use euler_rs::math::is_pandigital;

/// We shall say that an n-digit number is pandigital if it makes use of all the digits 1 to n exactly once; for example, the 5-digit number, 15234, is 1 through 5 pandigital.
///
/// The product 7254 is unusual, as the identity, 39 × 186 = 7254, containing multiplicand, multiplier, and product is 1 through 9 pandigital.
///
/// Find the sum of all products whose multiplicand/multiplier/product identity can be written as a 1 through 9 pandigital.
///
/// HINT: Some products can be obtained in more than one way so be sure to only include it once in your sum.
pub fn main() {
    let mut set: HashSet<u32> = HashSet::new();

    // to form a 1 - 9 pandigital number, the number of digits
    // in must by 9 across all of the mutiplicand, multiplier, and product
    // this means that if we start with a 1 digit number, the multiplier + product
    // must have 8 digits

    // 9 * 8765 = 78885 which has 10 digits, so the multiplicand could be 1 digit
    // this also means that the multiplier cannot exceed 10000ish, because as the
    // multiplicand gets larger the multiplier must get smaller or we'll produce
    // a number too large

    // if the multiplicand = 123, then the smallest remaining multiplier is 4567 = 561741 which has too many digits
    // 123 x 456 = 56088 which has 10 digits. this means that the multiplicand cannot exceed 99 (2 digits)

    // so the multiplicand range is 1 - 99 inclusive
    for multiplicand in 2u32..100 {
        // if the multiplicand has 1 digit, then the max value of the multiplicand is 9.
        // 9 x 8 = 72 --> too small
        // 9 x 87 = 783 --> too small
        // 9 x 876 = 7884 --> too small
        // 9 x 8765 = 78885 --> too large
        // 5 x 1234 = 6170 --> 9 digits

        // this means that as long as the multiplicand has 1 digit, our starting multiplier is 1234

        // if the multiplicand has 2 digits, then
        // 12 * 3456 = 41724 --> too large
        // so our starting multiplier must be 3 digits, and the smallest 3 digit valid number is 123
        // 123 * 45 = 5535 --> 9 digits, so 3 digit multipliers are potentially valid

        let start: u32 = if multiplicand <= 9 { 1234 } else { 123 };
        for multiplier in start..(10000 / multiplicand) {
            let product = multiplicand * multiplier;
            let num = format!("{}{}{}", multiplicand, multiplier, product);
            if is_pandigital(&num) {
                set.insert(product);
            }
        }
    }

    let sum: u32 = set.iter().sum();
    println!("{}", sum);
}
