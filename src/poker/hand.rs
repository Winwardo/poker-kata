use super::card;

#[derive(Debug, PartialEq)]
pub struct Hand {
    cards: Vec<card::Card>,
}

impl Hand {
    pub fn new(cards: Vec<card::Card>) -> Hand {
        Hand { cards: cards }
    }
}
