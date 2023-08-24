use euler_rs::{bigint::num_bigint::digits, math::factorial};
use num_bigint::BigUint;

fn main() {
    let sum: BigUint = (10u128..=100000)
        .filter_map(|i| {
            let i = BigUint::from(i);
            let digits = digits(&i);
            let factorial_sum: BigUint = digits.map(|i| factorial(i)).sum();
            if factorial_sum == i {
                Some(i)
            } else {
                None
            }
        })
        .sum();

    print!("{}", sum);
}
