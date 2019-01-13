#[derive(Debug, PartialEq)]
pub enum Players {
    Black,
    White,
}

#[derive(Debug, PartialEq)]
pub enum WinType {
    HighCard,
}

#[derive(Debug, PartialEq)]
pub struct Winner {
    pub player: Players,
    pub win_type: WinType,
}

#[derive(Debug, PartialEq)]
pub struct ComparisonResult {
    pub winner: Option<Winner>,
}
