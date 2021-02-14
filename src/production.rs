use std::{convert::TryFrom, fmt};

use enum_iterator::IntoEnumIterator;

use crate::game_state::GameState;
use crate::player::Worker;
use crate::resource::Resource;
use crate::resource::ResourceAmount;

#[derive(Clone, Copy, Debug, IntoEnumIterator)]
pub enum ProductionItem {
    WorkerIron = 0,
    WorkerStone = 1,
}

impl ProductionItem {
    pub fn get_cost(&self) -> ResourceAmount {
        match self {
            ProductionItem::WorkerIron => {
                let mut cost = ResourceAmount::new();
                *cost.get_mut(Resource::Iron) = 100;
                cost
            }
            ProductionItem::WorkerStone => {
                let mut cost = ResourceAmount::new();
                *cost.get_mut(Resource::Stone) = 100;
                cost
            }
        }
    }

    pub fn produce(&self, state: &mut GameState) {
        match self {
            ProductionItem::WorkerIron | ProductionItem::WorkerStone => {
                let player = state.get_player_mut(0);
                if player.get_stockpile_mut().consume(&self.get_cost()) {
                    player.workers.push(Worker::new());
                }
            }
        }
    }
}

impl TryFrom<usize> for ProductionItem {
    type Error = ();

    fn try_from(v: usize) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(ProductionItem::WorkerIron),
            1 => Ok(ProductionItem::WorkerStone),
            _ => Err(()),
        }
    }
}

impl fmt::Display for ProductionItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
