use num_bigint::BigUint;

fn main() {
    let mut num = BigUint::from(2usize);
    num = num.pow(1000);

    println!(
        "{}",
        num.to_radix_be(10)
            .into_iter()
            .map(|d| <u8 as Into<u128>>::into(d))
            .sum::<u128>()
    );
}
