use std::collections::HashSet;

use euler_rs::generators::PentagonalNumberGenerator;
use itertools::Itertools;

fn main() {
    let pentagonal_numbers = PentagonalNumberGenerator::<u128>::new()
        .take_while(|n| *n < 10_000_000)
        .collect_vec();
    let lookup = to_lookup(&pentagonal_numbers);

    let mut answer = u128::MAX;
    for p1 in pentagonal_numbers.iter() {
        for p2 in pentagonal_numbers.iter() {
            if p1 <= p2 {
                break;
            }
            let sum = p1 + p2;
            let difference = p1 - p2;
            if lookup.contains(&sum) && lookup.contains(&difference) && difference < answer {
                println!("Found new answer: {}", difference);
                answer = difference;
            }
        }
    }

    print!("{}", answer);
}

fn to_lookup(nums: &Vec<u128>) -> HashSet<u128> {
    let mut lookup: HashSet<u128> = HashSet::new();
    lookup.extend(nums.iter());
    return lookup;
}
