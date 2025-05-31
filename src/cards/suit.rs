use std::{error::Error, fmt::Display, str::FromStr};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Suit {
    Club,
    Diamond,
    Heart,
    Spade,
}

impl Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Suit::Club => "C",
                Suit::Diamond => "D",
                Suit::Heart => "H",
                Suit::Spade => "S",
            }
        )
    }
}

impl FromStr for Suit {
    type Err = SuitParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "C" => Ok(Suit::Club),
            "D" => Ok(Suit::Diamond),
            "H" => Ok(Suit::Heart),
            "S" => Ok(Suit::Spade),
            _ => Err(SuitParseError {
                invalid_char: s.to_owned(),
            }),
        }
    }
}

#[derive(Debug)]
pub struct SuitParseError {
    invalid_char: String,
}

impl Error for SuitParseError {}

impl Display for SuitParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Invalid character while parsing suit: {}",
            self.invalid_char
        )
    }
}
