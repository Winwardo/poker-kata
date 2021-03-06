#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum CardRank {
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

impl CardRank {
    pub fn to_string(&self) -> &str {
        match *self {
            CardRank::Two => "2",
            CardRank::Three => "3",
            CardRank::Four => "4",
            CardRank::Five => "5",
            CardRank::Six => "6",
            CardRank::Seven => "7",
            CardRank::Eight => "8",
            CardRank::Nine => "9",
            CardRank::Ten => "10",
            CardRank::Jack => "Jack",
            CardRank::Queen => "Queen",
            CardRank::King => "King",
            CardRank::Ace => "Ace",
        }
    }
}
