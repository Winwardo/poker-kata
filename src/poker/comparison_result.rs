use super::card_rank::*;

#[derive(Debug, PartialEq)]
pub enum Players {
    Black,
    White,
}

#[derive(Debug, PartialEq)]
pub enum WinType {
    HighCard(CardRank),
    Pair(CardRank),
    PairHighCard(CardRank),
}

#[derive(Debug, PartialEq)]
pub struct Winner {
    pub player: Players,
    pub win_type: WinType,
}

#[derive(Debug, PartialEq)]
pub struct ComparisonResult {
    pub winner: Option<Winner>,
    _s: (),
}

impl ComparisonResult {
    pub fn make(player: Players, win_type: WinType) -> ComparisonResult {
        ComparisonResult {
            winner: Some(Winner {
                player: player,
                win_type: win_type,
            }),
            _s: (),
        }
    }

    pub fn tie() -> ComparisonResult {
        ComparisonResult {
            winner: None,
            _s: (),
        }
    }
}
