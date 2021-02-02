use std::{convert::TryInto, fmt};
use tui::{
    layout::Constraint,
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, BorderType, Borders, Cell, Row, Table},
};

use crate::player::Player;
use crate::player::WorkerAction;
use crate::resource::Resource;

#[derive(Debug)]
pub struct GameState {
    players: Vec<Player>,
    paused: bool,
}

const TABLE_COLS: usize = Resource::count() + 1;
const TABLE_WIDTHS: &[Constraint] = &[Constraint::Ratio(1, TABLE_COLS as u32); TABLE_COLS];

impl GameState {
    pub fn new(num_players: u8) -> Self {
        let mut players = Vec::new();
        for id in 0u8..num_players {
            players.push(Player::new(id));
        }
        GameState {
            players,
            paused: false,
        }
    }

    pub fn step(&mut self) {
        for p in self.players.iter_mut() {
            p.step();
        }
    }

    pub fn resources_as_table(&self) -> Table {
        let header: Vec<_> = std::iter::once(Cell::from("Player Id"))
            .chain(Resource::names().map(Cell::from))
            .collect();
        let content = self.players.iter().map(|p| {
            let mut row = Vec::new();
            row.push(p.get_id().to_string());
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

    pub fn player_workers_as_table(&self, player: u8) -> Table {
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
        let active_workers = (0..Resource::count()).into_iter().map(|i| {
            let res = <_ as TryInto<Resource>>::try_into(i).unwrap();
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

    pub fn deallocate_player_worker(&mut self, player: u8, r: Resource) -> bool {
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

    pub fn allocate_player_worker(&mut self, player: u8, r: Resource) -> bool {
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

    pub fn toggle_paused(&mut self) {
        self.paused = !self.paused;
    }

    pub fn is_paused(&self) -> bool {
        self.paused
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
