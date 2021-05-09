use std::{convert::TryFrom, fmt};

use enum_iterator::IntoEnumIterator;

use crate::resource::{Resource, ResourceAmount};

#[derive(Clone, Copy, Debug, IntoEnumIterator)]
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
    items: [Trade; SellItem::VARIANT_COUNT],
}

impl ConsumerSector {
    pub fn get_trade(&self, item: SellItem) -> &Trade {
        &self.items[item as usize]
    }
}

impl Default for ConsumerSector {
    fn default() -> Self {
        let mut ret = ConsumerSector {
            items: [Trade::default(); SellItem::VARIANT_COUNT],
        };
        for item in SellItem::into_enum_iter() {
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
