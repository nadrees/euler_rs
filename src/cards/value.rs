use std::{error::Error, fmt::Display, str::FromStr};

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy)]
pub enum Value {
    Two = 2,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Value::Two => "2",
                Value::Three => "3",
                Value::Four => "4",
                Value::Five => "5",
                Value::Six => "6",
                Value::Seven => "7",
                Value::Eight => "8",
                Value::Nine => "9",
                Value::Ten => "T",
                Value::Jack => "J",
                Value::Queen => "Q",
                Value::King => "K",
                Value::Ace => "A",
            }
        )
    }
}

impl FromStr for Value {
    type Err = ValueParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "2" => Ok(Value::Two),
            "3" => Ok(Value::Three),
            "4" => Ok(Value::Four),
            "5" => Ok(Value::Five),
            "6" => Ok(Value::Six),
            "7" => Ok(Value::Seven),
            "8" => Ok(Value::Eight),
            "9" => Ok(Value::Nine),
            "T" => Ok(Value::Ten),
            "J" => Ok(Value::Jack),
            "Q" => Ok(Value::Queen),
            "K" => Ok(Value::King),
            "A" => Ok(Value::Ace),
            _ => Err(ValueParseError {
                invalid_char: s.to_owned(),
            }),
        }
    }
}

#[derive(Debug)]
pub struct ValueParseError {
    invalid_char: String,
}

impl Error for ValueParseError {}

impl Display for ValueParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Invalid character while parsing value: {}",
            self.invalid_char
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_sort() {
        let mut values = [Value::Five, Value::Three];
        values.sort();

        assert_eq!([Value::Three, Value::Five], values);
    }
}
