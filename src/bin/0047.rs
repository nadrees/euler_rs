use euler_rs::factors::PrimeFactors;
use itertools::Itertools;

pub fn main() {
    for (first, second, third, fourth) in (0..=u128::MAX).tuple_windows() {
        if PrimeFactors::distinct_count(first) == 4
            && PrimeFactors::distinct_count(second) == 4
            && PrimeFactors::distinct_count(third) == 4
            && PrimeFactors::distinct_count(fourth) == 4
        {
            println!("{}", first);
            break;
        }
    }
}
