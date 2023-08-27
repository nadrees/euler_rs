use euler_rs::is_palindrome;

fn main() {
    let answer: u64 = (1..1000000u64)
        .filter(|i| {
            is_palindrome(format!("{}", &i).as_str()) && is_palindrome(format!("{:b}", i).as_str())
        })
        .sum();
    print!("{}", answer);
}
