fn main() {
    let sum = (3..1000).filter(|x| x % 3 == 0 || x % 5 == 0).sum::<i64>();
    println!("{}", sum);
}
