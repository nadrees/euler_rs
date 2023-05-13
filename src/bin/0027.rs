use euler_rs::{
    cached_iterator::CachedIterator,
    generators::{Primes, QuadraticFormula},
};
use itertools::Itertools;

pub fn main() {
    let mut greatest_product = 0;
    let mut greatest_count = 0;
    let mut primes = CachedIterator::new(Primes::new());
    for a in -999..=999 {
        for b in -1000..=1000 {
            let qf: QuadraticFormula<i128> = QuadraticFormula::new(a, b);
            let count = qf
                .filter(|product| *product > 1)
                .map(|product| product.try_into().unwrap())
                .take_while(|product| {
                    primes
                        .iter()
                        .take_while(|prime| prime <= product)
                        .contains(product)
                })
                .collect::<Vec<_>>()
                .len();
            if count > greatest_count {
                greatest_count = count;
                greatest_product = a * b;
                println!(
                    "Found new greatest product: {} * {} = {} with {} primes",
                    a, b, greatest_product, greatest_count
                );
            }
        }
    }
    print!("{}", greatest_product);
}
