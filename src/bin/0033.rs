use euler_rs::math::Fraction;

pub fn main() {
    let mut cancelling_fractions: Vec<Fraction<u32>> = Vec::new();
    for numerator in 11..=99 {
        if numerator % 10 == 0 {
            continue;
        }
        let numerator_tens = numerator / 10;

        let denominator_tens = (numerator % 10) * 10;
        for denominator in (denominator_tens + 1)..=(denominator_tens + 9) {
            let denominator_ones = denominator % 10;
            let expected_fraction = Fraction::new(numerator_tens, denominator_ones);
            if expected_fraction >= 1.into() {
                continue;
            }

            let fraction = Fraction::new(numerator, denominator);
            if fraction == expected_fraction {
                cancelling_fractions.push(fraction);
            }
        }
    }
    let product = cancelling_fractions
        .iter()
        .product::<Fraction<u32>>()
        .reduce();
    print!("{}", product.denominator);
}
