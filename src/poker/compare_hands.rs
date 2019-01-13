use super::comparison_result;
use super::hand;

pub struct CompareHands {
    pub black: hand::Hand,
    pub white: hand::Hand,
}

impl CompareHands {
    pub fn compare(&self) -> comparison_result::ComparisonResult {
        comparison_result::ComparisonResult {
            winner: comparison_result::Players::Black,
            win_type: comparison_result::WinType::HighCard,
        }
    }
}
