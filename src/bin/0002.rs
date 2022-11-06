use euler_rs::generators::Fibonacci;
use num_bigint::BigUint;
use num_traits::Zero;

const MAX: u128 = 4000000;

fn main() {
    let max: BigUint = MAX.into();
    let zero: BigUint = Zero::zero();

    let fib = Fibonacci::new();
    let sum = fib
        .take_while(|x| x <= &max)
        .filter(|x| x % 2u8 == zero)
        .sum::<BigUint>();
    println!("{}", sum);
}
