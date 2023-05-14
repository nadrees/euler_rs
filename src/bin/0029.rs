use std::collections::HashSet;

use num_bigint::BigInt;

pub fn main() {
    let mut terms: HashSet<BigInt> = HashSet::new();
    for a in 2u8..=100 {
        for b in 2u8..=100 {
            terms.insert(BigInt::from(a).pow(b.into()));
        }
    }
    print!("{}", terms.len())
}
