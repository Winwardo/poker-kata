// mod card_rank {
// pub enum CardRank
// };

// use crate::{
//     poker: {
//         card_rank
//     }
// }

use crate::poker::card_rank;
// use super::card_rank;
use super::suits;

#[derive(Debug)]
pub struct Card {
    pub rank: card_rank::CardRank,
    pub suit: suits::Suit,
}
