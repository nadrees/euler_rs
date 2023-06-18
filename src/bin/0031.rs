pub fn main() {
    let mut count = 0;

    for two_pounds in 0..=1 {
        let one_pound_values = CoinValues {
            two_pounds,
            ..Default::default()
        };
        for one_pound in one_pound_values.get_counter(100) {
            let fifty_pence_values = CoinValues {
                one_pound,
                ..one_pound_values
            };
            for fifty_pence in fifty_pence_values.get_counter(50) {
                let twenty_pence_values = CoinValues {
                    fifty_pence,
                    ..fifty_pence_values
                };
                for twenty_pence in twenty_pence_values.get_counter(20) {
                    let ten_pence_values = CoinValues {
                        twenty_pence,
                        ..twenty_pence_values
                    };
                    for ten_pence in ten_pence_values.get_counter(10) {
                        let five_pence_values = CoinValues {
                            ten_pence,
                            ..ten_pence_values
                        };
                        for five_pence in five_pence_values.get_counter(5) {
                            let two_pence_value = CoinValues {
                                five_pence,
                                ..five_pence_values
                            };
                            for _ in two_pence_value.get_counter(2) {
                                // we can fill in the remaining amount using pennies. no need
                                // to calculate this
                                count += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    println!("{}", count);
}

struct CoinValues {
    two_pounds: usize,
    one_pound: usize,
    fifty_pence: usize,
    twenty_pence: usize,
    ten_pence: usize,
    five_pence: usize,
    two_pence: usize,
    one_penny: usize,
}

impl CoinValues {
    fn current_amount(&self) -> usize {
        self.two_pounds * 200
            + self.one_pound * 100
            + self.fifty_pence * 50
            + self.twenty_pence * 20
            + self.ten_pence * 10
            + self.five_pence * 5
            + self.two_pence * 2
            + self.one_penny * 1
    }

    fn get_counter(&self, amount: usize) -> impl Iterator<Item = usize> {
        let current_amount = self.current_amount();
        let count = (200 - current_amount) / amount;
        0..=count
    }
}

impl Default for CoinValues {
    fn default() -> Self {
        Self {
            two_pounds: Default::default(),
            one_pound: Default::default(),
            fifty_pence: Default::default(),
            twenty_pence: Default::default(),
            ten_pence: Default::default(),
            five_pence: Default::default(),
            two_pence: Default::default(),
            one_penny: Default::default(),
        }
    }
}
