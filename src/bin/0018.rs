use clap::Parser;
use euler_rs::{read_lines, triangle::maximum_sum_path};

#[derive(Debug, Parser)]
struct Args {
    file: String,
}

fn main() {
    let lines = parse_file();
    let answer = maximum_sum_path(&lines);
    println!("{}", answer);
}

fn parse_file() -> Vec<Vec<u8>> {
    let mut nums: Vec<Vec<u8>> = vec![];
    let args = Args::parse();
    if let Ok(lines) = read_lines(args.file) {
        for line in lines {
            if let Ok(line) = line {
                let line_nums = line
                    .split(" ")
                    .map(|s| s.parse().unwrap())
                    .collect::<Vec<_>>();
                nums.push(line_nums);
            }
        }
    }
    return nums;
}
