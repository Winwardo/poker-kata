use super::card_rank;
use super::suits;

#[derive(Debug, PartialEq)]
pub struct Card {
    pub rank: card_rank::CardRank,
    pub suit: suits::Suit,
}
