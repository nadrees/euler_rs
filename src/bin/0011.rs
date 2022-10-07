use std::{cmp::max, vec};

use clap::Parser;
use euler_rs::read_lines;

#[derive(Debug, Parser)]
struct Args {
    file: String,
}

fn main() {
    let args = Args::parse();
    let mut nums: Vec<Vec<u128>> = vec![];
    if let Ok(lines) = read_lines(args.file) {
        for line in lines {
            if let Ok(line) = line {
                nums.push(line.split(" ").map(|word| word.parse().unwrap()).collect());
            }
        }
    }

    let mut largest_product = 0;
    for row in 0..nums.len() {
        for column in 0..nums[row].len() {
            if column <= nums[row].len() - 4 {
                largest_product = max(
                    largest_product,
                    nums[row][column]
                        * nums[row][column + 1]
                        * nums[row][column + 2]
                        * nums[row][column + 3],
                );
            }

            if row <= nums.len() - 4 {
                largest_product = max(
                    largest_product,
                    nums[row][column]
                        * nums[row + 1][column]
                        * nums[row + 2][column]
                        * nums[row + 3][column],
                );
            }

            if column <= nums[row].len() - 4 && row <= nums.len() - 4 {
                largest_product = max(
                    largest_product,
                    nums[row][column]
                        * nums[row + 1][column + 1]
                        * nums[row + 2][column + 2]
                        * nums[row + 3][column + 3],
                );
            }
        }
    }

    println!("{}", largest_product);
}
