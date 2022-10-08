use num_bigint::BigUint;

/// Computes n!
pub fn factorial(n: u128) -> BigUint {
    let mut num: BigUint = BigUint::from(1u8);
    for i in 1u128..=n {
        num *= i;
    }
    return num;
}
