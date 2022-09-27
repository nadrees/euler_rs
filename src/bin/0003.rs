use euler_rs::prime_factors::prime_factors;

fn main() {
    let prime_factors = prime_factors(600851475143).max().unwrap();
    println!("{}", prime_factors);
}
