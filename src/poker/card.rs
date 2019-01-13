use super::card_rank::*;
use super::suits::*;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Card {
    rank: CardRank,
    suit: Suit,
}

impl Card {
    pub fn new(rank: CardRank, suit: Suit) -> Card {
        Card {
            rank: rank,
            suit: suit,
        }
    }

    pub fn from_string(input: &str) -> Option<Card> {
        None
    }
}
