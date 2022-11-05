use clap::Parser;
use euler_rs::read_lines;
use phf::phf_map;

static LETTER_SCORES: phf::Map<char, u16> = phf_map! {
  'A' => 1,
  'B' => 2,
  'C' => 3,
  'D' => 4,
  'E' => 5,
  'F' => 6,
  'G' => 7,
  'H' => 8,
  'I' => 9,
  'J' => 10,
  'K' => 11,
  'L' => 12,
  'M' => 13,
  'N' => 14,
  'O' => 15,
  'P' => 16,
  'Q' => 17,
  'R' => 18,
  'S' => 19,
  'T' => 20,
  'U' => 21,
  'V' => 22,
  'W' => 23,
  'X' => 24,
  'Y' => 25,
  'Z' => 26,
};

#[derive(Debug, Parser)]
struct Args {
    file: String,
}

fn main() {
    let lines = parse_file();
    let result: u128 = lines
        .iter()
        .enumerate()
        .map(|(i, line)| {
            let name_score = compute_name_score(line) as u128;
            return name_score * ((i + 1) as u128);
        })
        .sum();
    println!("{}", result);
}

fn compute_name_score(name: &str) -> u16 {
    name.chars()
        .map(|c| LETTER_SCORES.get(&c).unwrap_or(&0))
        .sum()
}

fn parse_file() -> Vec<String> {
    let mut all_lines: Vec<String> = vec![];
    let args = Args::parse();
    if let Ok(lines) = read_lines(args.file) {
        for line in lines {
            if let Ok(line) = line {
                all_lines.push(line);
            }
        }
    }
    all_lines.sort();
    return all_lines;
}
