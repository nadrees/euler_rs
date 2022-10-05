use euler_rs::primes::Primes;

fn main() {
    let primes: u128 = Primes::new(2000000).sum();
    println!("{}", primes);
}
