use std::cmp;

use super::card_rank::*;
use super::comparison_result::*;
use super::hand::*;

#[derive(Debug, PartialEq)]
pub struct CompareHands {
    pub black: Hand,
    pub white: Hand,
}

impl CompareHands {
    pub fn compare(&self) -> ComparisonResult {
        // Check for tie {
        let highest_rank_black = &self.black.cards.iter().max().unwrap();
        let highest_rank_white = &self.white.cards.iter().max().unwrap();

        println!("black, {:?}", highest_rank_black);
        println!("white, {:?}", highest_rank_white);

        if highest_rank_black > highest_rank_white {
            println!("black wins");
            ComparisonResult {
                winner: Some(Winner {
                    player: Players::Black,
                    win_type: WinType::HighCard,
                }),
            }
        } else if highest_rank_black < highest_rank_white {
            println!("white wins");

            ComparisonResult {
                winner: Some(Winner {
                    player: Players::White,
                    win_type: WinType::HighCard,
                }),
            }
        } else {
            println!("didnrt get a winner, tie??");
            ComparisonResult { winner: None }
        }
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

    #[test]
    fn black_has_high_card() {
        let input = CompareHands {
            black: Hand::new(vec![
                Card::new(CardRank::Two, Suit::Hearts),
                Card::new(CardRank::Three, Suit::Hearts),
                Card::new(CardRank::Four, Suit::Hearts),
                Card::new(CardRank::Five, Suit::Hearts),
                Card::new(CardRank::Ace, Suit::Hearts),
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

        assert_eq!(
            input,
            ComparisonResult {
                winner: Some(Winner {
                    player: Players::Black,
                    win_type: WinType::HighCard
                })
            }
        );
    }
}
