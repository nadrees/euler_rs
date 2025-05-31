use std::{error::Error, fmt::Display, str::FromStr};

use itertools::Itertools;
use suit::Suit;
use value::Value;

pub mod poker;
pub mod suit;
pub mod value;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Card {
    pub suit: Suit,
    pub value: Value,
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value.cmp(&other.value)
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.value, self.suit)
    }
}

impl FromStr for Card {
    type Err = CardParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 2 {
            return Err(CardParseError {
                message: format!("Cards must be exactly 2 characters long, received {}", s),
                inner_message: None,
            });
        }
        let (first, second) = s.split_at(s.char_indices().collect_vec()[1].0);
        let value = first.parse::<Value>().map_err(into_card_parse_error)?;
        let suit = second.parse::<Suit>().map_err(into_card_parse_error)?;
        Ok(Card { suit, value })
    }
}

fn into_card_parse_error(e: impl Error) -> CardParseError {
    CardParseError {
        message: "Error parsing card".to_string(),
        inner_message: Some(format!("{}", e)),
    }
}

#[derive(Debug)]
pub struct CardParseError {
    message: String,
    inner_message: Option<String>,
}

impl Error for CardParseError {}

impl Display for CardParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)?;
        if let Some(inner_message) = &self.inner_message {
            write!(f, ": {}", inner_message)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_parse() -> Result<(), CardParseError> {
        assert_eq!(
            "3S".parse::<Card>()?,
            Card {
                suit: Suit::Spade,
                value: Value::Three,
            }
        );
        Ok(())
    }
}
