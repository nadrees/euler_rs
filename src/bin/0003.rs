use euler_rs::factors::PrimeFactors;

fn main() {
    let prime_factors = PrimeFactors::new(600851475143).max().unwrap();
    println!("{}", prime_factors);
}
