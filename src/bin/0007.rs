use euler_rs::primes::Primes;

fn main() {
    let answer = Primes::new(u128::MAX).take(10001).last().unwrap();
    println!("{}", answer);
}
