use euler_rs::generators::Fibonacci;
use itertools::Itertools;

pub fn main() {
    let (answer, _) = Fibonacci::new_with_1_and_1()
        .find_position(|f| f.to_string().len() == 1000)
        .unwrap();
    println!("{}", answer + 1);
}
