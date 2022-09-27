fn main() {
    let answer = (2..=u128::MAX).find(|i| (2..=20).all(|d| *i % d == 0));
    println!("{}", answer.unwrap());
}
