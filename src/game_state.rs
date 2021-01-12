use std::{fmt, convert::TryInto};
use tui::{
    layout::Constraint,
    style::{Color, Style},
    widgets::{Block, Borders, BorderType, Cell, Row, Table},
};

use crate::player::Player;
use crate::resource::Resource;

#[derive(Debug)]
pub struct GameState {
    players: Vec<Player>,
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
            players
        }
    }

    pub fn step(&mut self) {
        for p in self.players.iter_mut() {
            p.step();
        }
    }

    pub fn as_table(&self) -> Table {
        let resource_names: Vec<_> = std::iter::once(Cell::from("Player Id"))
            .chain(
                (0..Resource::count())
                    .into_iter()
                    .map(|i| Cell::from(<_ as TryInto<Resource>>::try_into(i).unwrap().to_string())))
            .collect();
        let content = self.players.iter().map(| p | {
            let mut row = Vec::new();
            row.push(p.get_id().to_string());
            for r in p.get_stockpile().iter() {
                row.push(r.to_string());
            }
            row
        });
        let header = Row::new(resource_names);
        let rows = content
            .map(|mut r| {
                Row::new(r.drain(..).map(|c| Cell::from(c)))
            });
        Table::new(rows)
            .header(header)
            .widths(&TABLE_WIDTHS)
            .style(Style::default()
                .fg(Color::White))
            .block(Block::default().borders(Borders::ALL).border_type(BorderType::Thick).style(Style::default().bg(Color::DarkGray)))
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

