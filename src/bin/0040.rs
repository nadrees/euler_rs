use euler_rs::generators::ChampernownesConstant;

fn main() {
    let mut iterator = ChampernownesConstant::new();
    let mut answer = 1;
    for i in 1..=1000000 {
        let n = iterator.next().unwrap();
        if i == 1 || i == 10 || i == 100 || i == 1000 || i == 10000 || i == 100000 || i == 1000000 {
            answer *= n;
        }
    }
    print!("{}", answer);
}
