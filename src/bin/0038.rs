use euler_rs::math::is_pandigital;

/// What is the largest 1 to 9 pandigital 9-digit number that can be formed as the concatenated product
/// of an integer with (1,2,...,n) where n > 1?

fn main() {
    let mut num = 1u128;
    let mut max_pandigital_number = 0u128;
    loop {
        if let Some((count, str)) = maybe_panditigal_product(num) {
            if count == 1 {
                break;
            } else if is_pandigital(&str) {
                let pandigital_num: u128 = str.parse().unwrap();
                if pandigital_num > max_pandigital_number {
                    max_pandigital_number = pandigital_num;
                }
            }
        }
        num += 1;
    }
    print!("{}", max_pandigital_number);
}

fn maybe_panditigal_product(num: u128) -> Option<(usize, String)> {
    let mut product_parts = vec![];
    for product in products(num) {
        product_parts.push(product);
        let product_parts_digits_count: usize =
            product_parts.iter().map(|p| p.chars().count()).sum();
        if product_parts_digits_count == 9 {
            return Some((
                product_parts.len(),
                product_parts
                    .into_iter()
                    .reduce(|acc, next| format!("{}{}", acc, next))
                    .unwrap(),
            ));
        } else if product_parts_digits_count > 9 {
            return None;
        }
    }
    None
}

fn products(num: u128) -> impl Iterator<Item = String> {
    (1..=u128::MAX).map(move |i| format!("{}", i * num))
}
