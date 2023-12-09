use euler_rs::generators::pandigital_numbers_of_length;
use itertools::Itertools;

fn main() {
    let answer = pandigital_numbers_of_length(10)
        .filter(|n| {
            let s = format!("{:0>10}", n);
            let sub_str_nums = vec![
                &s[1..=3],
                &s[2..=4],
                &s[3..=5],
                &s[4..=6],
                &s[5..=7],
                &s[6..=8],
                &s[7..=9],
            ]
            .iter()
            .map(|s| s.parse::<u32>().unwrap())
            .collect_vec();
            sub_str_nums[0] % 2 == 0
                && sub_str_nums[1] % 3 == 0
                && sub_str_nums[2] % 5 == 0
                && sub_str_nums[3] % 7 == 0
                && sub_str_nums[4] % 11 == 0
                && sub_str_nums[5] % 13 == 0
                && sub_str_nums[6] % 17 == 0
        })
        .sum::<u128>();
    print!("{}", answer);
}
