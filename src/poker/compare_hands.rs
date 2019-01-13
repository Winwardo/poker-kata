use super::comparison_result::*;
use super::hand::*;

#[derive(Debug, PartialEq)]
pub struct CompareHands {
    pub black: Hand,
    pub white: Hand,
}

impl CompareHands {
    pub fn compare(&self) -> ComparisonResult {
        ComparisonResult { winner: None }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use super::super::card::*;
    use super::super::card_rank::*;
    use super::super::suits::*;

    #[test]
    fn same_hands_no_winner() {
        let input = CompareHands {
            black: Hand::new(vec![
                Card::new(CardRank::Two, Suit::Hearts),
                Card::new(CardRank::Three, Suit::Hearts),
                Card::new(CardRank::Four, Suit::Hearts),
                Card::new(CardRank::Five, Suit::Hearts),
                Card::new(CardRank::Six, Suit::Hearts),
            ])
            .unwrap(),
            white: Hand::new(vec![
                Card::new(CardRank::Two, Suit::Clubs),
                Card::new(CardRank::Three, Suit::Clubs),
                Card::new(CardRank::Four, Suit::Clubs),
                Card::new(CardRank::Five, Suit::Clubs),
                Card::new(CardRank::Six, Suit::Clubs),
            ])
            .unwrap(),
        }
        .compare();

        assert_eq!(input, ComparisonResult { winner: None });
    }
}
