use euler_rs::generators::{HexagonalNumberGenerator, PentagonalNumberGenerator, TriangleNumbers};

fn main() {
    let mut triangle_number_generator = TriangleNumbers::new();
    let mut pentagonal_number_generator = PentagonalNumberGenerator::<u128>::new();
    let mut hexagonal_number_generator = HexagonalNumberGenerator::<u128>::new();

    let mut last_triangle_number = triangle_number_generator.next().unwrap();
    let mut last_pentagonal_number = pentagonal_number_generator.next().unwrap();
    let mut last_hexagonal_number = hexagonal_number_generator.next().unwrap();

    while last_triangle_number <= 40755
        || last_triangle_number != last_pentagonal_number
        || last_triangle_number != last_hexagonal_number
    {
        if last_triangle_number < last_pentagonal_number
            && last_triangle_number < last_hexagonal_number
        {
            last_triangle_number = triangle_number_generator.next().unwrap();
        } else if last_pentagonal_number < last_hexagonal_number {
            last_pentagonal_number = pentagonal_number_generator.next().unwrap();
        } else {
            last_hexagonal_number = hexagonal_number_generator.next().unwrap();
        }
    }

    println!("{}", last_triangle_number);
}
