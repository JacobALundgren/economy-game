use std::{convert::TryFrom, fmt};

use enum_iterator::IntoEnumIterator;

use crate::game_state::GameState;
use crate::resource::{Resource, ResourceAmount};

#[derive(Clone, Copy, Debug, IntoEnumIterator)]
pub enum SellItem {
    Iron = 0,
    Stone = 1,
    Copper = 2,
}

pub struct Trade {
    pub give: ResourceAmount,
    pub receive: u64,
}

impl SellItem {
    pub fn get_trade(&self) -> Trade {
        match self {
            SellItem::Iron => {
                let mut cost = ResourceAmount::new();
                *cost.get_mut(Resource::Iron) = 100;
                Trade {
                    give: cost,
                    receive: 5,
                }
            }
            SellItem::Stone => {
                let mut cost = ResourceAmount::new();
                *cost.get_mut(Resource::Stone) = 100;
                Trade {
                    give: cost,
                    receive: 3,
                }
            }
            SellItem::Copper => {
                let mut cost = ResourceAmount::new();
                *cost.get_mut(Resource::Copper) = 100;
                Trade {
                    give: cost,
                    receive: 5,
                }
            }
        }
    }

    pub fn sell(&self, state: &mut GameState) {
        match self {
            SellItem::Iron | SellItem::Stone | SellItem::Copper => {
                let player = state.get_player_mut(0);
                let Trade {
                    give: cost,
                    receive: money,
                } = &self.get_trade();
                if player.get_stockpile_mut().consume(cost) {
                    player.add_money(*money);
                }
            }
        }
    }
}

impl TryFrom<usize> for SellItem {
    type Error = ();

    fn try_from(v: usize) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(SellItem::Iron),
            1 => Ok(SellItem::Stone),
            2 => Ok(SellItem::Copper),
            _ => Err(()),
        }
    }
}

impl fmt::Display for SellItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
