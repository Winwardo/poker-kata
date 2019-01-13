// mod poker::compare_hands;
use super::card::*;
use super::card_rank::*;
use super::compare_hands::*;
use super::comparison_result::*;
use super::hand::*;
use super::suits::*;

pub fn deserialize(input: &str) -> CompareHands {
    CompareHands {
        black: Hand::new(vec![]),
        white: Hand::new(vec![]),
    }
}

pub fn serialize(result: &ComparisonResult) -> String {
    String::from("White wins. - with high card: Ace")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_hand_deserializes() {
        let input = "Black: 2H 3D 5S 9C KD  White: 2C 3H 4S 8C AH";
        let expected = CompareHands {
            black: Hand::new(vec![
                Card {
                    rank: CardRank::Two,
                    suit: Suit::Hearts,
                },
                Card {
                    rank: CardRank::Three,
                    suit: Suit::Diamonds,
                },
                Card {
                    rank: CardRank::Five,
                    suit: Suit::Spades,
                },
                Card {
                    rank: CardRank::Nine,
                    suit: Suit::Clubs,
                },
                Card {
                    rank: CardRank::King,
                    suit: Suit::Diamonds,
                },
            ]),

            white: Hand::new(vec![
                Card {
                    rank: CardRank::Two,
                    suit: Suit::Clubs,
                },
                Card {
                    rank: CardRank::Three,
                    suit: Suit::Hearts,
                },
                Card {
                    rank: CardRank::Four,
                    suit: Suit::Spades,
                },
                Card {
                    rank: CardRank::Eight,
                    suit: Suit::Clubs,
                },
                Card {
                    rank: CardRank::Ace,
                    suit: Suit::Hearts,
                },
            ]),
        };

        assert_eq!(expected, deserialize(&input));
    }
}
