use euler_rs::is_palindrome;

fn main() {
    let mut max_palindrome: u128 = 0;
    let mut range = (100..=999).collect::<Vec<_>>();
    range.reverse();

    for x in range.clone() {
        for y in range.clone() {
            let product = x * y;
            if product <= max_palindrome {
                break;
            } else if is_palindrome(&format!("{}", product)) {
                max_palindrome = product;
            }
        }
    }
    println!("{}", max_palindrome);
}
