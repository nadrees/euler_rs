use euler_rs::{factors::Factors, generators::TriangleNumbers};

fn main() {
    let answer = TriangleNumbers::new()
        .find(|tn| Factors::new(tn).count() > 500)
        .unwrap();
    println!("{}", answer);
}
