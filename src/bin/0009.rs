use euler_rs::pythagorus::PythagoreanTriplets;

fn main() {
    let triplet = PythagoreanTriplets::new()
        .find(|triplet| triplet.a + triplet.b + triplet.c == 1000)
        .unwrap();
    println!("{}", triplet.a * triplet.b * triplet.c);
}
