use super::card_rank::*;
use super::suits::*;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Card {
    rank: CardRank,
    suit: Suit,
}

#[derive(Debug, PartialEq)]
pub enum CardError {
    InvalidSuit(char),
    InvalidRank(char),
    WrongLength,
}

impl Card {
    pub fn new(rank: CardRank, suit: Suit) -> Card {
        Card {
            rank: rank,
            suit: suit,
        }
    }

    pub fn from_string(input: &str) -> Result<Card, CardError> {
        let mut chars = input.chars();
        let count = input.chars().count();

        if count == 2 {
            let rank_char = chars.next().unwrap();
            let rank = match rank_char {
                '2' => CardRank::Two,
                '3' => CardRank::Three,
                '4' => CardRank::Four,
                '5' => CardRank::Five,
                '6' => CardRank::Six,
                '7' => CardRank::Seven,
                '8' => CardRank::Eight,
                '9' => CardRank::Nine,
                'T' => CardRank::Ten,
                'J' => CardRank::Jack,
                'Q' => CardRank::Queen,
                'K' => CardRank::King,
                'A' => CardRank::Ace,
                x => return Err(CardError::InvalidRank(x)),
            };

            let suit_char = chars.next().unwrap();
            let suit = match suit_char {
                'C' => Suit::Clubs,
                'D' => Suit::Diamonds,
                'H' => Suit::Hearts,
                'S' => Suit::Spades,
                x => return Err(CardError::InvalidSuit(x)),
            };

            Ok(Card {
                rank: rank,
                suit: suit,
            })
        } else {
            Err(CardError::WrongLength)
        }
    }
}
