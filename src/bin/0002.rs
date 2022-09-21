use euler_rs::fibonacci::Fibonacci;

const MAX: u64 = 4000000;

fn main() {
    let fib = Fibonacci::new();
    let sum = fib
        .take_while(|x| x <= &MAX)
        .filter(|x| x % 2 == 0)
        .sum::<u64>();
    println!("{}", sum);
}
