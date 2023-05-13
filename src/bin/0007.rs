use euler_rs::generators::Primes;

fn main() {
    let answer = Primes::new().take(10001).last().unwrap();
    println!("{}", answer);
}
