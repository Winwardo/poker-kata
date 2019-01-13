// mod poker::compare_hands;
use super::card;
use super::card_rank;
use super::compare_hands;
use super::comparison_result;
use super::hand;
use super::suits;

pub fn deserialize(input: &str) -> compare_hands::CompareHands {
    compare_hands::CompareHands {
        black: hand::Hand::new(vec![]),
        white: hand::Hand::new(vec![]),
    }
}

pub fn serialize(result: &comparison_result::ComparisonResult) -> String {
    String::from("White wins. - with high card: Ace")
}

// #[cfg(test)]
// pub mod tests {
//     // Note this useful idiom: importing names from outer (for mod tests) scope.
//     extern crate rstest;
//     use rstest::rstest_parametrize;

//     use super::*;

//     #[rstest_parametrize(
//         expected,
//         input,
//         case(
//             "Black: 2H 3D 5S 9C KD  White: 2C 3H 4S 8C AH",
// compare_hands::CompareHands {
//     black: hand::Hand { cards: vec![] },
//     white: hand::Hand { cards: vec![] },
// }
//         )
//     )]
//     fn deserialize_test(expected: compare_hands::CompareHands, input: &str) {
//         assert_eq!(expected, deserialize(&input));
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_hand_deserializes() {
        let input = "Black: 2H 3D 5S 9C KD  White: 2C 3H 4S 8C AH";
        let expected = compare_hands::CompareHands {
            black: hand::Hand::new(vec![
                card::Card {
                    rank: card_rank::CardRank::Two,
                    suit: suits::Suit::Hearts,
                },
                card::Card {
                    rank: card_rank::CardRank::Three,
                    suit: suits::Suit::Diamonds,
                },
                card::Card {
                    rank: card_rank::CardRank::Five,
                    suit: suits::Suit::Spades,
                },
                card::Card {
                    rank: card_rank::CardRank::Nine,
                    suit: suits::Suit::Clubs,
                },
                card::Card {
                    rank: card_rank::CardRank::King,
                    suit: suits::Suit::Diamonds,
                },
            ]),

            white: hand::Hand::new(vec![
                card::Card {
                    rank: card_rank::CardRank::Two,
                    suit: suits::Suit::Clubs,
                },
                card::Card {
                    rank: card_rank::CardRank::Three,
                    suit: suits::Suit::Hearts,
                },
                card::Card {
                    rank: card_rank::CardRank::Four,
                    suit: suits::Suit::Spades,
                },
                card::Card {
                    rank: card_rank::CardRank::Eight,
                    suit: suits::Suit::Clubs,
                },
                card::Card {
                    rank: card_rank::CardRank::Ace,
                    suit: suits::Suit::Hearts,
                },
            ]),
        };

        assert_eq!(expected, deserialize(&input));
    }
}
