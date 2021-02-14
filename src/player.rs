use std::fmt;

use crate::resource::Resource;
use crate::resource::ResourceAmount;

#[derive(Debug, PartialEq)]
pub enum WorkerAction {
    Gather(Resource),
    Idle,
}

#[derive(Debug)]
pub struct Worker {
    pub current_action: WorkerAction,
}

impl Worker {
    pub fn new() -> Self {
        Worker {
            current_action: WorkerAction::Gather(Resource::Iron),
        }
    }
}

#[derive(Debug)]
pub struct Player {
    id: u8,
    pub workers: Vec<Worker>,
    stockpile: ResourceAmount,
}

impl Player {
    pub fn new(id: u8) -> Self {
        Player {
            id,
            workers: vec![Worker::new(), Worker::new(), Worker::new()],
            stockpile: ResourceAmount::new(),
        }
    }

    pub fn step(&mut self) {
        for w in self.workers.iter() {
            match &w.current_action {
                WorkerAction::Gather(r) => *self.stockpile.get_mut(*r) += 1,
                WorkerAction::Idle => (),
            }
        }
    }

    pub fn get_id(&self) -> u8 {
        self.id
    }

    pub fn get_stockpile(&self) -> &ResourceAmount {
        &self.stockpile
    }

    pub fn get_stockpile_mut(&mut self) -> &mut ResourceAmount {
        &mut self.stockpile
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}: {}", self.id, self.stockpile)
    }
}
