use super::comparison_result::*;
use super::hand::*;

#[derive(Debug, PartialEq)]
pub struct CompareHands {
    pub black: Hand,
    pub white: Hand,
}

impl CompareHands {

    pub fn compare(&self) -> ComparisonResult {
        ComparisonResult {
            winner: Players::Black,
            win_type: WinType::HighCard,
        }
    }
}
