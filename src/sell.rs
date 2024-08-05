use std::{convert::TryFrom, fmt};

use enum_iterator::Sequence;
use rand::thread_rng;
use rand_distr::{Distribution, Normal};

use crate::resource::{Resource, ResourceAmount};

#[derive(Clone, Copy, Debug, Sequence)]
pub enum SellItem {
    Iron = 0,
    Stone = 1,
    Copper = 2,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Trade {
    pub give: ResourceAmount,
    pub receive: u64,
}

#[derive(Debug)]
pub struct ConsumerSector {
    items: [Trade; enum_iterator::cardinality::<SellItem>()],
}

fn update_trade(trade: &Trade) -> Trade {
    let mut rng = thread_rng();
    let normal_dist = Normal::new(-0.01, 0.01).unwrap();
    let val: f64 = normal_dist.sample(&mut rng);
    Trade {
        receive: ((trade.receive as f64) * val.exp()) as u64,
        ..*trade
    }
}

impl ConsumerSector {
    pub fn get_trade(&self, item: SellItem) -> &Trade {
        &self.items[item as usize]
    }

    pub fn trade(&mut self, stockpile: &mut ResourceAmount, item: SellItem) -> Option<u64> {
        let trade = &mut self.items[item as usize];
        let mut money = None;
        if stockpile.consume(&trade.give) {
            money = Some(trade.receive);
            *trade = update_trade(trade);
        }
        money
    }
}

impl Default for ConsumerSector {
    fn default() -> Self {
        let mut ret = ConsumerSector {
            items: [Trade::default(); enum_iterator::cardinality::<SellItem>()],
        };
        for item in enum_iterator::all::<SellItem>() {
            ret.items[item as usize] = item.get_default_trade();
        }
        ret
    }
}

impl SellItem {
    fn get_default_trade(&self) -> Trade {
        match self {
            SellItem::Iron => {
                let mut cost = ResourceAmount::new();
                *cost.get_mut(Resource::Iron) = 100;
                Trade {
                    give: cost,
                    receive: 500,
                }
            }
            SellItem::Stone => {
                let mut cost = ResourceAmount::new();
                *cost.get_mut(Resource::Stone) = 100;
                Trade {
                    give: cost,
                    receive: 300,
                }
            }
            SellItem::Copper => {
                let mut cost = ResourceAmount::new();
                *cost.get_mut(Resource::Copper) = 100;
                Trade {
                    give: cost,
                    receive: 500,
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
