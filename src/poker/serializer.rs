use super::card::*;
use super::card_rank::*;
use super::compare_hands::*;
use super::comparison_result::*;
use super::hand::*;
use super::suits::*;

#[derive(Debug, PartialEq)]
pub enum CompareHandsError {
    Other,
}

pub fn deserialize(input: &str) -> Result<CompareHands, CompareHandsError> {
    Err(CompareHandsError::Other)
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
                Card::new(CardRank::Two, Suit::Hearts),
                Card::new(CardRank::Three, Suit::Diamonds),
                Card::new(CardRank::Five, Suit::Spades),
                Card::new(CardRank::Nine, Suit::Clubs),
                Card::new(CardRank::King, Suit::Diamonds),
            ])
            .unwrap(),
            white: Hand::new(vec![
                Card::new(CardRank::Two, Suit::Clubs),
                Card::new(CardRank::Three, Suit::Hearts),
                Card::new(CardRank::Four, Suit::Spades),
                Card::new(CardRank::Eight, Suit::Clubs),
                Card::new(CardRank::Ace, Suit::Hearts),
            ])
            .unwrap(),
        };

        // Err("no")
        assert_eq!(Ok(expected), deserialize(&input));
    }
}
