use std::{fmt, convert::{TryFrom, TryInto}};
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Resource {
    Iron = 0,
    Copper = 1,
    Stone = 2,
}

impl Resource {
    pub const fn count() -> usize {
        (Resource::Stone as usize) + 1
    }

    pub fn names() -> impl Iterator<Item=String> {
        (0..Resource::count())
            .into_iter()
            .map(|i| <_ as TryInto<Resource>>::try_into(i).unwrap().to_string())
    }
}
 
impl TryFrom<usize> for Resource {
    type Error = ();

    fn try_from(v: usize) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Resource::Iron),
            1 => Ok(Resource::Copper),
            2 => Ok(Resource::Stone),
            _ => Err(())
        }
    }
}

impl fmt::Display for Resource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

