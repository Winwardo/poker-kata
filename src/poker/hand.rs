use super::card;

#[derive(Debug, PartialEq)]
pub struct Hand {
    cards: Vec<card::Card>,
}

#[derive(Debug)]
pub enum HandError {
    InvalidHand,
}

impl Hand {
    pub fn new(cards: Vec<card::Card>) -> Result<Hand, HandError> {
        Ok(Hand { cards: cards })
    }
}
