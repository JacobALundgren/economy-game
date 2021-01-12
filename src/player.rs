use std::{fmt, convert::TryInto};

use crate::resource::Resource;

#[derive(Debug)]
enum WorkerAction {
    Gather(Resource),
}

#[derive(Debug)]
struct Worker {
    current_action: WorkerAction,
}

impl Worker {
    fn new() -> Self {
        Worker { current_action: WorkerAction::Gather(Resource::Iron) }
    }
}

#[derive(Debug)]
pub struct Stockpile {
    res: [u32; Resource::count()]
}

impl Stockpile {
    fn new() -> Self {
        Stockpile { res: [0; Resource::count()] }
    }

    fn get(&mut self, res: Resource) -> &mut u32 {
        &mut self.res[res as usize]
    }

    pub fn iter(&self) -> std::slice::Iter<u32> {
        self.res.iter()
    }
}

impl fmt::Display for Stockpile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.res.len() {
            if let Ok(name) = <_ as TryInto<Resource>>::try_into(i) {
                write!(f, "{}: {}\t", name.to_string(), self.res[i])?;
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct Player {
    id: u8,
    workers: Vec<Worker>,
    stockpile: Stockpile,
}

impl Player {
    pub fn new(id: u8) -> Self {
        Player{id, workers: vec![Worker::new(), Worker::new(), Worker::new()], stockpile: Stockpile::new()}
    }

    pub fn step(&mut self) {
        for w in self.workers.iter() {
            match &w.current_action {
                WorkerAction::Gather(r) => *self.stockpile.get(*r) += 1,
            }
        }
    }

    pub fn get_id(&self) -> u8 {
        self.id
    }

    pub fn get_stockpile(&self) -> &Stockpile {
        &self.stockpile
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}\n", self.id, self.stockpile)
    }
}


