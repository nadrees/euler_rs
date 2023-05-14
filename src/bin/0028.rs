use euler_rs::generators::SpiralDiagonalNumbers;

pub fn main() {
    let generator = SpiralDiagonalNumbers::new(1001);
    let answer: i128 = generator.sum();
    print!("{}", answer);
}
