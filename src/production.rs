use std::{convert::TryFrom, fmt};

use enum_iterator::Sequence;

use crate::game_state::Duration;
use crate::player::{Player, Worker};
use crate::resource::Resource;
use crate::resource::ResourceAmount;

#[derive(Clone, Copy, Debug, Sequence)]
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

    pub fn get_production_time(&self) -> Duration {
        match self {
            ProductionItem::WorkerIron => std::time::Duration::from_secs(8).into(),
            ProductionItem::WorkerStone => std::time::Duration::from_secs(5).into(),
        }
    }

    pub fn produce(&self, player: &mut Player) {
        match self {
            ProductionItem::WorkerIron | ProductionItem::WorkerStone => {
                player.workers.push(Worker::new());
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
