fn main() {
    println!("Hello, world!");
}

mod poker;

enum CardRank {
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    Ace = 14,
}

struct Card {
    rank: CardRank,
    suit: poker::suits::Suit,
}

struct Hand {
    cards: Vec<Card>,
}

enum Players {
    Black,
    White,
}

enum WinType {
    HighCard,
}

struct CompareHands {
    black: Hand,
    white: Hand,
}

struct ComparisonResult {
    winner: Players,
    win_type: WinType,
}

impl CompareHands {
    fn compare(&self) -> ComparisonResult {
        ComparisonResult {
            winner: Players::Black,
            win_type: WinType::HighCard,
        }
    }
}

fn deserialize(input: &str) -> CompareHands {
    CompareHands {
        black: Hand { cards: vec![] },
        white: Hand { cards: vec![] },
    }
}

fn serialize(result: &ComparisonResult) -> String {
    String::from("White wins. - with high card: Ace")
}

#[allow(dead_code)]
fn poker_hand(input: &str) -> String {
    serialize(&deserialize(input).compare())
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    extern crate rstest;
    use rstest::rstest_parametrize;

    use super::*;

    #[rstest_parametrize(
        expected,
        input,
        case(
            "White wins. - with high card: Ace",
            "Black: 2H 3D 5S 9C KD  White: 2C 3H 4S 8C AH"
        ),
        case(
            "Black wins. - with full house: 4 over 2",
            "Black: 2H 4S 4C 2D 4H  White: 2S 8S AS QS 3S"
        ),
        case(
            "Black wins. - with high card: 9",
            "Black: 2H 3D 5S 9C KD  White: 2C 3H 4S 8C KH"
        ),
        case("Tie.", "Black: 2H 3D 5S 9C KD  White: 2D 3H 5C 9S KH")
    )]
    fn poker_hand_test(expected: &str, input: &str) {
        assert_eq!(expected, poker_hand(input));
    }
}
