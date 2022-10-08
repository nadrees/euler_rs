use clap::Parser;
use euler_rs::read_lines;
use num_bigint::BigUint;

#[derive(Debug, Parser)]
struct Args {
    file: String,
}

fn main() {
    let nums = parse_file();
    let result: BigUint = nums.into_iter().sum();
    let str_result = &format!("{}", result)[0..10];
    println!("{}", str_result);
}

fn parse_file() -> Vec<BigUint> {
    let mut nums: Vec<BigUint> = vec![];
    let args = Args::parse();
    if let Ok(lines) = read_lines(args.file) {
        for line in lines {
            if let Ok(line) = line {
                nums.push(BigUint::parse_bytes(line.as_bytes(), 10).unwrap());
            }
        }
    }
    return nums;
}
