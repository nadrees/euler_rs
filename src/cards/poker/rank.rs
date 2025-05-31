use itertools::Itertools;

use crate::cards::{suit::Suit, value::Value, Card};

pub enum Rank<'a> {
    HighCard(&'a Card),
    OnePair(PairDetails<'a>),
    TwoPair(PairDetails<'a>, PairDetails<'a>),
    ThreeOfAKind(ThreeOfAKindDetails<'a>),
    Straight(StraightDetails<'a>),
    Flush(FlushDetails<'a>),
    FullHouse {
        three_of_a_kind: ThreeOfAKindDetails<'a>,
        pair: PairDetails<'a>,
    },
    FourOfAKind {
        value: Value,
    },
    StraightFlush {
        suit: Suit,
        min_value: Value,
        max_value: Value,
    },
    RoyalFlush {
        suit: Suit,
    },
}

pub fn is_straight<'a>(cards: [&'a Card; 5]) -> bool {
    let mut last_value = &cards[0].value;
    for card in &cards[1..] {
        let value = &card.value;
        if (*value as u8) != ((*last_value as u8) + 1) {
            return false;
        }
        last_value = value;
    }
    return true;
}

pub fn is_flush<'a>(cards: [&'a Card; 5]) -> bool {
    cards.iter().into_group_map_by(|c| &c.suit).keys().len() == 1
}

pub struct PairDetails<'a> {
    pub cards: [&'a Card; 2],
}

impl<'a> PairDetails<'a> {
    pub fn new(cards: [&'a Card; 2]) -> Self {
        assert!(
            cards.iter().into_group_map_by(|c| &c.value).len() == 1,
            "All cards must have the same value, received {}",
            cards.iter().map(|c| &c.value).join(", ")
        );
        Self { cards }
    }

    pub fn value(&self) -> &Value {
        &self.cards[0].value
    }
}

pub struct ThreeOfAKindDetails<'a> {
    pub cards: [&'a Card; 3],
}

impl<'a> ThreeOfAKindDetails<'a> {
    pub fn new(cards: [&'a Card; 3]) -> Self {
        assert!(
            cards.iter().into_group_map_by(|c| &c.value).len() == 1,
            "All cards must have the same value, received {}",
            cards.iter().map(|c| &c.value).join(", ")
        );
        Self { cards }
    }

    pub fn value(&self) -> &Value {
        &self.cards[0].value
    }
}

pub struct StraightDetails<'a> {
    pub cards: [&'a Card; 5],
}

impl<'a> StraightDetails<'a> {
    pub fn new(cards: [&'a Card; 5]) -> Self {
        assert!(
            is_straight(cards),
            "Straights must be consecutive values, found: {}",
            cards.iter().join(", ")
        );
        StraightDetails { cards }
    }

    pub fn low_card(&self) -> &'a Card {
        &self.cards[0]
    }

    pub fn high_card(&self) -> &'a Card {
        &self.cards[4]
    }
}

pub struct FlushDetails<'a> {
    cards: [&'a Card; 5],
}

impl<'a> FlushDetails<'a> {
    pub fn new(cards: [&'a Card; 5]) -> Self {
        assert!(
            is_flush(cards),
            "All cards must be of the same suit, received: {}",
            cards.iter().join(", ")
        );
        Self { cards }
    }
}

impl<'a> FlushDetails<'a> {
    pub fn suit(&self) -> &Suit {
        &self.cards[0].suit
    }
}

#[cfg(test)]
mod tests {
    use yare::parameterized;

    use super::*;

    #[parameterized(
        straight = { [&"3D".parse().unwrap(), &"4D".parse().unwrap(), &"5D".parse().unwrap(), &"6D".parse().unwrap(), &"7H".parse().unwrap()], true},
        not_staight = { [&"4D".parse().unwrap(), &"4H".parse().unwrap(), &"5H".parse().unwrap(), &"6C".parse().unwrap(), &"7C".parse().unwrap()], false}
    )]
    fn test_is_straight(cards: [&Card; 5], expected: bool) {
        assert_eq!(is_straight(cards), expected);
    }
}
