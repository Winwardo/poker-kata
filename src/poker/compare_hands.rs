use super::card::*;
use super::comparison_result::*;
use super::hand::*;

#[derive(Debug, PartialEq)]
pub struct CompareHands {
    pub black: Hand,
    pub white: Hand,
}

fn find_pair(cards: &Vec<Card>) -> Option<(&Card, &Card)> {
    cards.windows(2).fold(None, |acc, slice| {
        if acc.is_none() {
            let left = &slice[0];
            let right = &slice[1];

            if left.rank == right.rank {
                return Some((&left, &right));
            }
        }

        acc
    })
}

impl CompareHands {
    pub fn compare(&self) -> ComparisonResult {
        let mut sorted_black = self.black.cards.clone();
        sorted_black.sort();
        sorted_black.reverse();

        let mut sorted_white = self.white.cards.clone();
        sorted_white.sort();
        sorted_white.reverse();

        // Check for a pair
        {
            let black_pair = find_pair(&sorted_black);
            let white_pair = find_pair(&sorted_white);

            if black_pair.is_some() {
                if white_pair.is_some() {
                    let (b1, _b2) = black_pair.unwrap();
                    let (w1, _w2) = white_pair.unwrap();

                    if b1 > w1 {
                        return ComparisonResult::make(Players::Black, WinType::Pair(b1.rank));
                    } else {
                        // find the largest non-pair card

                    }
                } else {
                    let (b1, _b2) = black_pair.unwrap();
                    return ComparisonResult::make(Players::Black, WinType::Pair(b1.rank));
                }
            } else {
                if white_pair.is_some() {
                    let (w1, _w2) = white_pair.unwrap();
                    return ComparisonResult::make(Players::White, WinType::Pair(w1.rank));
                }
            }
        }

        // Check for tie
        {
            let highest_rank_black = &sorted_black.iter().max().unwrap();
            let highest_rank_white = &sorted_white.iter().max().unwrap();

            if highest_rank_black > highest_rank_white {
                return ComparisonResult::make(
                    Players::Black,
                    WinType::HighCard(highest_rank_black.rank),
                );
            } else if highest_rank_black < highest_rank_white {
                return ComparisonResult::make(
                    Players::White,
                    WinType::HighCard(highest_rank_white.rank),
                );
            } else {
                return ComparisonResult::tie();
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

        assert_eq!(input, ComparisonResult::tie());
    }

    #[test]
    fn high_card_black() {
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
            ComparisonResult::make(Players::Black, WinType::HighCard(CardRank::Ace))
        );
    }

    #[test]
    fn pair_black_beats_white_high_card() {
        let input = CompareHands {
            black: Hand::new(vec![
                Card::new(CardRank::Two, Suit::Hearts),
                Card::new(CardRank::Two, Suit::Diamonds),
                Card::new(CardRank::Four, Suit::Hearts),
                Card::new(CardRank::Five, Suit::Hearts),
                Card::new(CardRank::Six, Suit::Hearts),
            ])
            .unwrap(),
            white: Hand::new(vec![
                Card::new(CardRank::Two, Suit::Clubs),
                Card::new(CardRank::Three, Suit::Spades),
                Card::new(CardRank::Four, Suit::Clubs),
                Card::new(CardRank::Five, Suit::Clubs),
                Card::new(CardRank::Seven, Suit::Clubs),
            ])
            .unwrap(),
        }
        .compare();

        assert_eq!(
            input,
            ComparisonResult::make(Players::Black, WinType::Pair(CardRank::Two))
        );
    }

    #[test]
    fn pair_both_white_high_card_that_isnt_in_pair_wins() {
        // let input = CompareHands {
        //     black: Hand::new(vec![
        //         Card::new(CardRank::Ace, Suit::Hearts),
        //         Card::new(CardRank::Ace, Suit::Diamonds),
        //         Card::new(CardRank::Four, Suit::Hearts),
        //         Card::new(CardRank::Five, Suit::Hearts),
        //         Card::new(CardRank::Six, Suit::Hearts),
        //     ])
        //     .unwrap(),
        //     white: Hand::new(vec![
        //         Card::new(CardRank::Ace, Suit::Clubs),
        //         Card::new(CardRank::Ace, Suit::Spades),
        //         Card::new(CardRank::Four, Suit::Clubs),
        //         Card::new(CardRank::Five, Suit::Clubs),
        //         Card::new(CardRank::Seven, Suit::Clubs),
        //     ])
        //     .unwrap(),
        // }
        // .compare();

        // assert_eq!(
        //     input,
        //     ComparisonResult::make(Players::White, WinType::PairHighCard(CardRank::Seven))
        // );
    }
}
