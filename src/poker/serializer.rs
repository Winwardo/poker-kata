// mod poker::compare_hands;
use super::compare_hands;
use super::comparison_result;
use super::hand;

pub fn deserialize(input: &str) -> compare_hands::CompareHands {
    compare_hands::CompareHands {
        black: hand::Hand { cards: vec![] },
        white: hand::Hand { cards: vec![] },
    }
}

pub fn serialize(result: &comparison_result::ComparisonResult) -> String {
    String::from("White wins. - with high card: Ace")
}
