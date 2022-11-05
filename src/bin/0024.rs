use itertools::Itertools;

pub fn main() {
    let answer = (0..=9).permutations(10).nth(999999).unwrap();
    let s = answer.iter().map(|i| i.to_string()).join("");
    println!("{}", s);
}
