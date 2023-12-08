use clap::Parser;

use euler_rs::{
    alpha::english_str_indexed_value, cached_iterator::CachedIterator, generators::TriangleNumbers,
    read_lines,
};
use itertools::Itertools;

#[derive(Debug, Parser)]
struct Args {
    file: String,
}

fn main() {
    let mut triangle_numbers = CachedIterator::new(TriangleNumbers::new());

    let args = Args::parse();
    let lines = read_file(args.file);
    let answer = lines
        .iter()
        .map(|line| english_str_indexed_value(line) as u128)
        .filter(|v| triangle_numbers.iter().take_while(|n| n <= v).contains(v))
        .count();
    print!("{}", answer);
}

fn read_file(file: String) -> Vec<String> {
    let mut words = vec![];
    if let Ok(lines) = read_lines(file) {
        for line in lines {
            if let Ok(line) = line {
                let mut line_words = line
                    .split(',')
                    .map(|w| w.trim_matches('"').to_owned())
                    .collect_vec();
                words.append(&mut line_words);
            }
        }
    }
    words
}
