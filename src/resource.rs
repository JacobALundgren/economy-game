use std::{
    convert::{TryFrom, TryInto},
    fmt,
};

use enum_iterator::IntoEnumIterator;

#[derive(Clone, Copy, Debug, IntoEnumIterator, PartialEq)]
pub enum Resource {
    Iron = 0,
    Copper = 1,
    Stone = 2,
}

impl Resource {
    pub const fn count() -> usize {
        (Resource::Stone as usize) + 1
    }

    pub fn names() -> impl Iterator<Item = String> {
        Self::into_enum_iter().map(|res| res.to_string())
    }
}

impl TryFrom<usize> for Resource {
    type Error = ();

    fn try_from(v: usize) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Resource::Iron),
            1 => Ok(Resource::Copper),
            2 => Ok(Resource::Stone),
            _ => Err(()),
        }
    }
}

impl fmt::Display for Resource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ResourceAmount {
    res: [u32; Resource::VARIANT_COUNT],
}

impl ResourceAmount {
    pub fn new() -> Self {
        ResourceAmount {
            res: [0; Resource::VARIANT_COUNT],
        }
    }

    pub fn consume(&mut self, amount: &ResourceAmount) -> bool {
        if !self.has_available(amount) {
            return false;
        }
        for i in 0..self.res.len() {
            self.res[i] -= amount.res[i];
        }
        true
    }

    pub fn get(&self, res: Resource) -> u32 {
        self.res[res as usize]
    }

    pub fn get_mut(&mut self, res: Resource) -> &mut u32 {
        &mut self.res[res as usize]
    }

    fn has_available(&self, query: &ResourceAmount) -> bool {
        Resource::into_enum_iter().all(|res| query.get(res) <= self.get(res))
    }

    pub fn iter(&self) -> std::slice::Iter<u32> {
        self.res.iter()
    }
}

impl fmt::Display for ResourceAmount {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.res.len() {
            if let Ok(name) = <_ as TryInto<Resource>>::try_into(i) {
                write!(f, "{}: {}\t", name.to_string(), self.res[i])?;
            }
        }
        Ok(())
    }
}
