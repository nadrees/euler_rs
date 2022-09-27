fn main() {
    let sum_of_squares: u128 = (1..=100).map(|x| x * x).sum();

    let mut square_of_sums: u128 = (1..=100).sum();
    square_of_sums *= square_of_sums;

    let answer = square_of_sums - sum_of_squares;
    println!("{}", answer);
}
