use euler_rs::bigint::num_bigint::digits;
use itertools::Itertools;
use num_bigint::{BigUint, ToBigUint};

pub fn main() {
    let mut num = BigUint::new(vec![0]);
    for n in 1..=1000 {
        num += n.to_biguint().unwrap().pow(n);
    }
    let mut digits = digits(&num).collect_vec();
    digits.reverse();
    let digits = digits.iter().take(10).rev().join("");
    println!("{}", digits);
}
