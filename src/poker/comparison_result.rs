pub enum Players {
    Black,
    White,
}

pub enum WinType {
    HighCard,
}

pub struct ComparisonResult {
    pub winner: Players,
    pub win_type: WinType,
}
