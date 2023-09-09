// p = a + b + c, where p is perimeter, (a,b,c) are lengths of sides of triangle
// a^2 + b^2 = c^2 -- triangle is right triangle

// eliminate c:
// c = p - a - b
// a^2 + b^2 = (p - a - b)^2
// a^2 + b^2 = p^2 + a^2 + b^2 - 2pa - 2pb - 2ab

// eliminate common terms on both sides:
// a^2 = p^2 + a^2 - 2pa - 2pb - 2ab
// 0 = p^2 - 2pa - 2pb - 2ab

// solve for b, given a and p:
// 2pb - 2ab = p^2 - 2pa
// pb - ab = (p^2 - 2pa) / 2
// b(p - a) = (p^2 - 2pa) / 2
// b = (p^2 - 2pa) / 2(p - a)
// b = (p^2 - 2pa) / (2p - 2a)

fn main() {
    let mut max_num_solutions = 0;
    let mut max_p = 0;
    for p in 2..=1000 {
        let mut num_solutions = 0;
        for a in 2..=p {
            let numerator = p * p - 2 * p * a;
            let denominator = 2 * p - 2 * a;
            if denominator != 0 && numerator % denominator == 0 {
                num_solutions += 1;
            }
        }

        if num_solutions > max_num_solutions {
            max_num_solutions = num_solutions;
            max_p = p;
        }
    }

    print!("{}", max_p);
}
