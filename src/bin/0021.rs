use euler_rs::math::get_amicable_partner;

fn main() {
    let answer = (1..10000)
        .filter(|i| get_amicable_partner(i).is_some())
        .sum::<u128>();
    println!("{}", answer);
}
