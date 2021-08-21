use std::{collections::VecDeque, fmt};

use crate::game_state::Duration;
use crate::production::ProductionItem;
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
            current_action: WorkerAction::Idle,
        }
    }
}

pub type PlayerId = u8;

#[derive(Debug)]
pub struct Player {
    id: PlayerId,
    pub workers: Vec<Worker>,
    stockpile: ResourceAmount,
    money: u64,
    production_queue: VecDeque<(ProductionItem, Duration)>,
}

impl Player {
    pub fn new(id: PlayerId) -> Self {
        Player {
            id,
            workers: vec![Worker::new(), Worker::new(), Worker::new()],
            stockpile: ResourceAmount::new(),
            money: 0,
            production_queue: VecDeque::new(),
        }
    }

    pub fn step(&mut self) {
        let completed_item = self
            .production_queue
            .front_mut()
            .and_then(|production_item| {
                let production_time = &mut production_item.1;
                debug_assert!(production_time.ticks != 0);
                production_time.ticks -= 1;
                if production_time.ticks == 0 {
                    Some(production_item.0)
                } else {
                    None
                }
            });

        if let Some(item) = completed_item {
            self.production_queue.pop_front();
            item.produce(self);
        }

        for w in self.workers.iter() {
            match &w.current_action {
                WorkerAction::Gather(r) => *self.stockpile.get_mut(*r) += 1,
                WorkerAction::Idle => (),
            }
        }
    }

    pub fn get_id(&self) -> PlayerId {
        self.id
    }

    pub fn get_money(&self) -> u64 {
        self.money
    }

    pub fn get_stockpile(&self) -> &ResourceAmount {
        &self.stockpile
    }

    pub fn get_stockpile_mut(&mut self) -> &mut ResourceAmount {
        &mut self.stockpile
    }

    pub fn add_money(&mut self, amount: u64) {
        self.money += amount
    }

    pub fn enqueue_production(&mut self, item: ProductionItem, production_time: Duration) {
        self.production_queue.push_back((item, production_time));
    }

    pub fn get_current_production(&self) -> Option<&(ProductionItem, Duration)> {
        self.production_queue.front()
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}: {}", self.id, self.stockpile)
    }
}
