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
    player: Players,
    win_type: WinType,
}

#[derive(Debug, PartialEq)]
pub struct ComparisonResult {
    pub winner: Option<Winner>,
}
