use euler_rs::generators::Primes;

fn main() {
    let primes: u128 = Primes::new_up_to(2000000).sum();
    println!("{}", primes);
}
