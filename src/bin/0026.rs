use euler_rs::math::{cycle_length, Fraction};

pub fn main() {
    let mut current_longest_i: Option<u16> = None;
    let mut current_longest_length: Option<usize> = None;
    for i in 2u16..=1000 {
        if let Some(cycle_length) = cycle_length(Fraction::new(1, i)) {
            if cycle_length > current_longest_length.unwrap_or(0) {
                current_longest_i = Some(i);
                current_longest_length = Some(cycle_length);
            }
        }
    }
    println!("{}", current_longest_i.unwrap());
}
