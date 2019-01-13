use std::collections::BTreeSet;
use std::iter::FromIterator;

use super::card::*;

#[derive(Debug, PartialEq)]
pub struct Hand {
    cards: Vec<Card>,
}

#[derive(Debug, PartialEq)]
pub enum HandError {
    NotEnoughCards,
    TooManyCards,
    DuplicatedCards,
    InvalidHand,
}

impl Hand {
    pub fn new(cards: Vec<Card>) -> Result<Hand, HandError> {
        match cards.len() {
            x if x < 5 => Err(HandError::NotEnoughCards),
            x if x > 5 => Err(HandError::TooManyCards),
            _ => {
                let r = BTreeSet::from_iter(cards.clone());

                Ok(Hand { cards: cards })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use super::super::card_rank::*;
    use super::super::suits::*;

    #[test]
    fn simple_hand_works() {
        let input = Hand::new(vec![
            Card::new(CardRank::Two, Suit::Hearts),
            Card::new(CardRank::Three, Suit::Hearts),
            Card::new(CardRank::Four, Suit::Hearts),
            Card::new(CardRank::Five, Suit::Hearts),
            Card::new(CardRank::Six, Suit::Hearts),
        ]);

        assert!(input.is_ok());
    }

    #[test]
    fn not_enough_cards() {
        let input = Hand::new(vec![
            Card::new(CardRank::Two, Suit::Hearts),
            Card::new(CardRank::Three, Suit::Hearts),
            Card::new(CardRank::Four, Suit::Hearts),
            Card::new(CardRank::Five, Suit::Hearts),
        ]);

        assert_eq!(Err(HandError::NotEnoughCards), input);
    }

    #[test]
    fn too_many_cards() {
        let input = Hand::new(vec![
            Card::new(CardRank::Two, Suit::Hearts),
            Card::new(CardRank::Three, Suit::Hearts),
            Card::new(CardRank::Four, Suit::Hearts),
            Card::new(CardRank::Five, Suit::Hearts),
            Card::new(CardRank::Six, Suit::Hearts),
            Card::new(CardRank::Seven, Suit::Hearts),
        ]);

        assert_eq!(Err(HandError::TooManyCards), input);
    }

    #[test]
    fn duplicate_cards() {
        let input = Hand::new(vec![
            Card::new(CardRank::Two, Suit::Hearts),
            Card::new(CardRank::Two, Suit::Hearts),
            Card::new(CardRank::Four, Suit::Hearts),
            Card::new(CardRank::Five, Suit::Hearts),
            Card::new(CardRank::Six, Suit::Hearts),
        ]);

        assert_eq!(Err(HandError::DuplicatedCards), input);
    }
}
