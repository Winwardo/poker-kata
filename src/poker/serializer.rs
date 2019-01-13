use regex::Regex;

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

#[derive(Debug, PartialEq)]
pub enum DeserializeError {
    CompareHands(CompareHandsError),
    BadFormat,
    InvalidCards,
    Other,
}

fn text_to_cards(text: &str) -> Vec<Card> {
    text.split(" ")
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|card_str| Card::from_string(card_str))
        .filter_map(|card| card.ok())
        .collect::<Vec<Card>>()
}

pub fn deserialize(input: &str) -> Result<CompareHands, DeserializeError> {
    println!("desering");
    let re = Regex::new(r"^Black: (.*)  White: (.*)$").unwrap();

    // let captures = re.captures(input);
    // if captures.unwrap().len() == 1 {
    // for cap in captures {
    //     let black_text = &cap[0];
    //     let white_text = &cap[1];

    //         let black_card = Card::from_string(black_text);
    //     }
    // }
    re.captures(input)
        .and_then(|captures| {
            // Did we match something after "Black" and after "White"?
            let black = captures.get(1);
            let white = captures.get(2);

            if black.is_some() && white.is_some() {
                Some((black.unwrap().as_str(), white.unwrap().as_str()))
            } else {
                None
            }
        })
        .ok_or(DeserializeError::BadFormat)
        .and_then(|matches| {
            let (black_text, white_text) = matches;

            let black_cards = text_to_cards(black_text);
            let white_cards = text_to_cards(white_text);

            println!("black cards? {:?}", black_cards);
            println!("white cards? {:?}", white_cards);

            Ok((black_cards, white_cards))
        })
        .and_then(|(black, white)| {
            let black_hand = Hand::new(black);
            let white_hand = Hand::new(white);
            if black_hand.is_ok() && white_hand.is_ok() {
                Ok(CompareHands {
                    black: black_hand.unwrap(),
                    white: white_hand.unwrap(),
                })
            } else {
                Err(DeserializeError::Other)
            }
        })
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

        assert_eq!(Ok(expected), deserialize(&input));
    }

    #[test]
    fn bad_format_errors() {
        let input = "Bad format";
        assert_eq!(Err(DeserializeError::BadFormat), deserialize(&input));
    }
}
