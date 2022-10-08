use euler_rs::bigint::num_bigint::digits;
use num_bigint::BigUint;

fn main() {
    let mut num = BigUint::from(2usize);
    num = num.pow(1000);

    println!("{}", digits(&num).sum::<u128>());
}
