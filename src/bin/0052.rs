use euler_rs::permutation::are_permutations;
use itertools::Itertools;

pub fn main() {
    for x in 1.. {
        let nums = [x, 2 * x, 3 * x, 4 * x, 5 * x, 6 * x]
            .into_iter()
            .map(|x| format!("{}", x))
            .collect_vec();
        if are_permutations(nums) {
            println!("{}", x);
            return;
        }
    }
}
