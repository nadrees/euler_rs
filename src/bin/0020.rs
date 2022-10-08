use euler_rs::{bigint::num_bigint::digits, math::factorial};

fn main() {
    println!("{}", digits(&factorial(100)).sum::<u128>());
}
