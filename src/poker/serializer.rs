use super::card::*;
use super::comparison_result::*;
use super::hand::*;

pub fn serialize(result: &ComparisonResult) -> Result<String, ()> {
    // result
    //     .winner
    //     .map(|x| String::from("White wins. - with high card: Ace"))
    //     .or(String::from("Tie."))

    if let Some(winner) = &result.winner {
        match winner.win_type {
            WinType::HighCard(card) => Ok(format!(
                "{player} wins. - with high card: {card}",
                player = winner.player.to_string(),
                card = card.to_string()
            )),
            _ => Err(()),
        }
    } else {
        Ok(String::from("Tie."))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use super::super::card_rank::*;

    #[test]
    fn white_wins_highcard_ace() {
        let input = ComparisonResult::make(Players::White, WinType::HighCard(CardRank::Ace));
        let expected = String::from("White wins. - with high card: Ace");

        assert_eq!(Ok(expected), serialize(&input));
    }

    #[test]
    fn black_wins_highcard_seven() {
        let input = ComparisonResult::make(Players::Black, WinType::HighCard(CardRank::Seven));
        let expected = String::from("Black wins. - with high card: 7");

        assert_eq!(Ok(expected), serialize(&input));
    }

    #[test]
    fn tie() {
        let input = ComparisonResult::tie();
        let expected = String::from("Tie.");

        assert_eq!(Ok(expected), serialize(&input));
    }
}
