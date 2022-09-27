use euler_rs::prime_factors::PrimeFactorIterator;

fn main() {
    let prime_factors = PrimeFactorIterator::new(600851475143).max().unwrap();
    println!("{}", prime_factors);
}
