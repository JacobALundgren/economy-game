use std::fmt;
use tui::{
    layout::Constraint,
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, BorderType, Borders, Cell, Row, Table},
};

use enum_iterator::IntoEnumIterator;

use crate::player::{Player, PlayerId, WorkerAction};
use crate::production::ProductionItem;
use crate::resource::Resource;
use crate::sell::{ConsumerSector, SellItem, Trade};

#[derive(Clone, Copy, Debug)]
pub enum GameAction {
    AllocateWorker(PlayerId, Resource),
    DeallocateWorker(PlayerId, Resource),
    TogglePause,
    Produce(PlayerId, ProductionItem),
    Sell(PlayerId, SellItem),
}

#[derive(Debug)]
pub struct GameState {
    players: Vec<Player>,
    paused: bool,
    consumer_sector: ConsumerSector,
}

const TABLE_COLS: usize = Resource::VARIANT_COUNT + 2;
const TABLE_WIDTHS: &[Constraint] = &[Constraint::Ratio(1, TABLE_COLS as u32); TABLE_COLS];

impl GameState {
    pub fn new() -> Self {
        GameState {
            players: Vec::new(),
            paused: false,
            consumer_sector: ConsumerSector::default(),
        }
    }

    pub fn step(&mut self) {
        for p in self.players.iter_mut() {
            p.step();
        }
    }

    pub fn get_player_mut(&mut self, player: PlayerId) -> &mut Player {
        &mut self.players[player as usize]
    }

    pub fn resources_as_table(&self) -> Table {
        let header: Vec<_> = std::iter::once(Cell::from("Player Id"))
            .chain(std::iter::once(Cell::from("Money")))
            .chain(Resource::names().map(Cell::from))
            .collect();
        let content = self.players.iter().map(|p| {
            let mut row = Vec::with_capacity(p.get_stockpile().iter().count() + 2);
            row.push(p.get_id().to_string());
            row.push(p.get_money().to_string());
            for r in p.get_stockpile().iter() {
                row.push(r.to_string());
            }
            row
        });
        let header = Row::new(header);
        let rows = content.map(|mut r| Row::new(r.drain(..).map(Cell::from)));
        Table::new(rows)
            .header(header)
            .widths(&TABLE_WIDTHS)
            .style(Style::default().fg(Color::White))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Thick)
                    .style(Style::default().bg(Color::DarkGray)),
            )
    }

    pub fn player_workers_as_table(&self, player: PlayerId) -> Table {
        let p = &self.players[player as usize];
        let idle_count = p
            .workers
            .iter()
            .filter(|w| w.current_action == WorkerAction::Idle)
            .count();
        let idle_row = std::iter::once(Row::new(vec![
            Cell::from("Idle"),
            Cell::from(idle_count.to_string()),
        ]));
        let active_workers = Resource::into_enum_iter().map(|res| {
            let count = p
                .workers
                .iter()
                .filter(|w| w.current_action == WorkerAction::Gather(res))
                .count();
            Row::new(vec![
                Cell::from(res.to_string()),
                Cell::from(count.to_string()),
            ])
        });
        Table::new(idle_row.chain(active_workers))
            .widths(&[Constraint::Percentage(80), Constraint::Percentage(20)])
            .style(Style::default())
            .block(
                Block::default()
                    .title(Span::styled(
                        "Workers",
                        Style::default()
                            .add_modifier(Modifier::BOLD)
                            .fg(Color::LightRed),
                    ))
                    .borders(Borders::ALL)
                    .border_type(BorderType::Thick)
                    .style(Style::default().bg(Color::DarkGray)),
            )
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol(">>")
    }

    fn deallocate_player_worker(&mut self, player: PlayerId, r: Resource) -> bool {
        let player = &mut self.players[player as usize];
        if let Some(worker) = player
            .workers
            .iter_mut()
            .find(|w| w.current_action == WorkerAction::Gather(r))
        {
            worker.current_action = WorkerAction::Idle;
            true
        } else {
            false
        }
    }

    fn allocate_player_worker(&mut self, player: PlayerId, r: Resource) -> bool {
        let player = &mut self.players[player as usize];
        if let Some(worker) = player
            .workers
            .iter_mut()
            .find(|w| w.current_action == WorkerAction::Idle)
        {
            worker.current_action = WorkerAction::Gather(r);
            true
        } else {
            false
        }
    }

    fn toggle_paused(&mut self) {
        self.paused = !self.paused;
    }

    pub fn is_paused(&self) -> bool {
        self.paused
    }

    pub fn get_sell_trade(&self, item: SellItem) -> &Trade {
        self.consumer_sector.get_trade(item)
    }

    pub fn register_player(&mut self) -> PlayerId {
        let id = self.players.len() as PlayerId;
        self.players.push(Player::new(id));
        id
    }

    fn produce(&mut self, player: PlayerId, item: ProductionItem) {
        item.produce(player, self);
    }

    fn sell(&mut self, player: PlayerId, item: SellItem) {
        let Self {
            ref mut consumer_sector,
            ref mut players,
            ..
        } = self;
        let player = &mut players[player as usize];
        if let Some(money) = consumer_sector.trade(player.get_stockpile_mut(), item) {
            player.add_money(money);
        }
    }

    pub fn handle_action(&mut self, action: GameAction) {
        match action {
            GameAction::AllocateWorker(player, resource) => {
                self.allocate_player_worker(player, resource);
            }
            GameAction::DeallocateWorker(player, resource) => {
                self.deallocate_player_worker(player, resource);
            }
            GameAction::TogglePause => {
                self.toggle_paused();
            }
            GameAction::Produce(player, item) => self.produce(player, item),
            GameAction::Sell(player, item) => self.sell(player, item),
        }
    }
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for p in self.players.iter() {
            write!(f, "{}", p)?;
        }
        Ok(())
    }
}
