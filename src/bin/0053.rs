use euler_rs::math::factorial;
use num_bigint::BigUint;

pub fn main() {
    let one_million = BigUint::from(1_000_000u128);

    let mut count = 0;
    for n in 23..=100 {
        let n_factorial = factorial(n);
        for r in 1..=n {
            let r_factorial = factorial(r);
            let n_minus_r_factorial = factorial(n - r);
            let combinations_count = n_factorial.clone() / (r_factorial * n_minus_r_factorial);
            if combinations_count > one_million {
                count += 1;
            }
        }
    }

    println!("{}", count);
}
