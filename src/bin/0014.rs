use euler_rs::generators::Collatz;

fn main() {
    let mut longest_chain_count = 0;
    let mut longest_chain_number = 0;

    for i in 1..1000000 {
        let collatz_length = Collatz::new(i).count();
        if collatz_length > longest_chain_count {
            longest_chain_count = collatz_length;
            longest_chain_number = i;
        }
    }

    println!("{}", longest_chain_number);
}
