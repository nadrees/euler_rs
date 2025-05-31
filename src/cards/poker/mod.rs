use std::str::FromStr;

use itertools::Itertools;
use rank::{
    is_flush, is_straight, FlushDetails, PairDetails, Rank, StraightDetails, ThreeOfAKindDetails,
};
use thiserror::Error;

use super::{value::Value, Card, CardParseError};

pub mod rank;

#[derive(Debug, Error)]
pub enum HandParseError {
    #[error("Error parsing a card: {0}")]
    CardParserError(#[from] CardParseError),
    #[error("A hand should have exactly 5 cards, received {0}")]
    CardCountError(usize),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Hand {
    cards: [Card; 5],
}

impl Hand {
    pub fn new(cards: [Card; 5]) -> Self {
        let mut cards = cards.clone();
        cards.sort();
        Self { cards }
    }

    pub fn rank(&self) -> Rank {
        let cards_as_refs = self.cards.each_ref();
        let suits = self.cards.iter().map(|c| c.suit).dedup().collect_vec();

        // Royal Flush
        if is_flush(cards_as_refs)
            && cards_as_refs[0].value == Value::Ten
            && cards_as_refs[1].value == Value::Jack
            && cards_as_refs[2].value == Value::Queen
            && cards_as_refs[3].value == Value::King
            && cards_as_refs[4].value == Value::Ace
        {
            return Rank::RoyalFlush { suit: suits[0] };
        }

        // Straight Flush
        if is_flush(cards_as_refs) && is_straight(cards_as_refs) {
            return Rank::StraightFlush {
                suit: suits[0],
                min_value: self.cards[0].value,
                max_value: self.cards[4].value,
            };
        }

        // Four of a kind
        let grouped_values = self.cards.iter().into_group_map_by(|c| c.value);
        for v in grouped_values.keys() {
            if grouped_values[v].len() == 4 {
                return Rank::FourOfAKind { value: *v };
            }
        }

        let three_of_a_kind_value = grouped_values.keys().find(|k| grouped_values[k].len() == 3);
        let pair_values = grouped_values
            .keys()
            .filter(|k| grouped_values[k].len() == 2)
            .collect_vec();

        // Full House
        if let Some(three_of_a_kind_value) = three_of_a_kind_value {
            if pair_values.len() == 1 {
                let three_of_a_kind_cards = &grouped_values[three_of_a_kind_value];
                let pair_cards = &grouped_values[pair_values[0]];
                return Rank::FullHouse {
                    three_of_a_kind: ThreeOfAKindDetails::new([
                        three_of_a_kind_cards[0],
                        three_of_a_kind_cards[1],
                        three_of_a_kind_cards[2],
                    ]),
                    pair: PairDetails::new([pair_cards[0], pair_cards[1]]),
                };
            }
        }

        // Flush
        if is_flush(cards_as_refs) {
            return Rank::Flush(FlushDetails::new(cards_as_refs));
        }

        // Straight
        if is_straight(cards_as_refs) {
            return Rank::Straight(StraightDetails::new(cards_as_refs));
        }

        // Three of a kind
        if let Some(three_of_a_kind_value) = three_of_a_kind_value {
            let three_of_a_kind_cards = &grouped_values[three_of_a_kind_value];
            return Rank::ThreeOfAKind(ThreeOfAKindDetails::new([
                three_of_a_kind_cards[0],
                three_of_a_kind_cards[1],
                three_of_a_kind_cards[2],
            ]));
        }

        // Two Pair
        if pair_values.len() == 2 {
            let high_pair_value = pair_values.iter().max().unwrap();
            let low_pair_value = pair_values.iter().min().unwrap();

            let high_pair_cards = &grouped_values[high_pair_value];
            let low_pair_cards = &grouped_values[low_pair_value];
            return Rank::TwoPair(
                PairDetails::new([high_pair_cards[0], high_pair_cards[1]]),
                PairDetails::new([low_pair_cards[0], low_pair_cards[1]]),
            );
        }

        // One Pair
        if pair_values.len() == 1 {
            let pair_cards = &grouped_values[pair_values[0]];
            return Rank::OnePair(PairDetails::new([pair_cards[0], pair_cards[1]]));
        }

        // High Card
        let high_card = self.cards.iter().sorted().last().unwrap();
        return Rank::HighCard(high_card);
    }
}

impl FromStr for Hand {
    type Err = HandParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cards = s
            .split(",")
            .map(|c| c.trim().parse::<Card>())
            .collect::<Result<Vec<_>, _>>()?;
        if cards.len() != 5 {
            return Err(HandParseError::CardCountError(cards.len()));
        }
        // unwrap safe here because of the length check above
        Ok(Hand::new([
            cards.pop().unwrap(),
            cards.pop().unwrap(),
            cards.pop().unwrap(),
            cards.pop().unwrap(),
            cards.pop().unwrap(),
        ]))
    }
}
