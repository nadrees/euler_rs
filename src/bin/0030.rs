/// We know this to be the upper limit because
/// 5*(9^5) = 295,245, which is > 99,999 -> so any
/// number with 5 digits could be equal to the sum
/// of 5th powers
///
/// 6*(9^5) = 354,294, which is < 999,999 -> so any
/// number with 7 or more digits will never be equal
/// to the sum of 5th powers
const MAX_VALUE: u32 = 354294;

pub fn main() {
    let mut sum = 0;
    for i in 2..=MAX_VALUE {
        let digits_sum: u32 = i
            .to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap().pow(5))
            .sum();
        if digits_sum == i {
            sum += digits_sum;
        }
    }
    print!("{}", sum);
}
