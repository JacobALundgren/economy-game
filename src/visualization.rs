use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    terminal::{Frame, Terminal},
    text::Span,
    widgets::{Block, BorderType, Borders, Cell, Paragraph, Row, Table, TableState, Tabs},
};
use std::{
    convert::{TryFrom, TryInto},
    fmt,
};

use enum_iterator::Sequence;

use crate::game_state::{GameAction, GameState};
use crate::input::InputAction;
use crate::player::PlayerId;
use crate::production::ProductionItem;
use crate::resource::Resource;
use crate::sell::{SellItem, Trade};

#[derive(Clone, Copy, Debug, Sequence, PartialEq)]
pub enum TabType {
    Resources = 0,
    Help = 1,
    Production = 2,
    Sell = 3,
}

impl TabType {
    pub fn get_hotkey(&self) -> u8 {
        match self {
            TabType::Resources => b'r',
            TabType::Help => b'h',
            TabType::Production => b'd',
            TabType::Sell => b's',
        }
    }
}

impl TryFrom<usize> for TabType {
    type Error = ();

    fn try_from(v: usize) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(TabType::Resources),
            1 => Ok(TabType::Help),
            2 => Ok(TabType::Production),
            3 => Ok(TabType::Sell),
            _ => Err(()),
        }
    }
}

impl fmt::Display for TabType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

trait Tab {
    fn draw(&mut self, f: &mut Frame, area: Rect, player: PlayerId, state: &GameState);
    fn handle_input(&mut self, player: PlayerId, input: InputAction) -> Option<GameAction>;
}

#[derive(Clone)]
struct WrappingTableState {
    state: TableState,
    min: usize,
    max: usize,
}

impl WrappingTableState {
    fn get_mut(&mut self) -> &mut TableState {
        &mut self.state
    }

    fn get_row(&self) -> usize {
        self.state.selected().unwrap()
    }

    fn next(&mut self) {
        if self.min == self.max {
            return;
        }
        let curr = self.state.selected().unwrap();
        self.state.select(Some(
            (curr + 1 - self.min).rem_euclid(self.max - self.min) + self.min,
        ));
    }

    fn new(min: usize, max: usize) -> Self {
        let mut ret = WrappingTableState {
            state: TableState::default(),
            min,
            max,
        };
        ret.state.select(Some(min));
        ret
    }

    fn prev(&mut self) {
        if self.min == self.max {
            return;
        }
        let curr = self.state.selected().unwrap();
        self.state.select(Some(
            (((curr - self.min) as i32 - 1).rem_euclid((self.max - self.min) as i32) as usize)
                + self.min,
        ));
    }
}

struct ResourceTab {
    worker_selected: WrappingTableState,
}

impl Default for ResourceTab {
    fn default() -> Self {
        ResourceTab {
            worker_selected: WrappingTableState::new(1, Resource::count() + 1),
        }
    }
}

impl Tab for ResourceTab {
    fn draw(&mut self, f: &mut Frame, area: Rect, player: PlayerId, state: &GameState) {
        let main_blocks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Length(area.width - 30), Constraint::Max(30)].as_ref())
            .split(area);
        let rt = state.resources_as_table();
        f.render_widget(rt, main_blocks[0]);
        let wt = state.player_workers_as_table(player);
        f.render_stateful_widget(wt, main_blocks[1], self.worker_selected.get_mut());
    }

    fn handle_input(&mut self, player: PlayerId, input: InputAction) -> Option<GameAction> {
        match input {
            InputAction::MoveUp => {
                self.worker_selected.prev();
                None
            }
            InputAction::MoveDown => {
                self.worker_selected.next();
                None
            }
            InputAction::Decrease => {
                let resource =
                    <_ as TryInto<Resource>>::try_into(self.worker_selected.get_row() - 1).unwrap();
                Some(GameAction::DeallocateWorker(player, resource))
            }
            InputAction::Increase => {
                let resource =
                    <_ as TryInto<Resource>>::try_into(self.worker_selected.get_row() - 1).unwrap();
                Some(GameAction::AllocateWorker(player, resource))
            }
            _ => None,
        }
    }
}

#[derive(Default)]
struct HelpTab {}

impl Tab for HelpTab {
    fn draw(&mut self, f: &mut Frame, area: Rect, _: PlayerId, _: &GameState) {
        let blocks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
            .split(area);
        let overview = Paragraph::new(concat!(
            "Control the allocation of your workers to different resources",
            " using the arrow keys. Balance your economy to produce what you need.",
            " Sell resources for money."
        ))
        .block(
            Block::default()
                .style(Style::default().bg(Color::DarkGray))
                .borders(Borders::ALL)
                .border_type(BorderType::Thick)
                .title(Span::from("Overview")),
        );
        f.render_widget(overview, blocks[0]);

        let hotkeys = [("p", "Toggle pause"), ("q", "Exit program")];
        let table = Table::new(
            hotkeys
                .iter()
                .map(|(key, desc)| Row::new(vec![Cell::from(*key), Cell::from(*desc)])),
            [Constraint::Percentage(10), Constraint::Percentage(90)].iter(),
        )
        .block(
            Block::default()
                .style(Style::default().bg(Color::DarkGray))
                .borders(Borders::ALL)
                .border_type(BorderType::Thick)
                .title(Span::from("Hotkeys")),
        );
        f.render_widget(table, blocks[1]);
    }

    fn handle_input(&mut self, _: PlayerId, _: InputAction) -> Option<GameAction> {
        None
    }
}

struct ProductionTab {
    selected: WrappingTableState,
}

impl Default for ProductionTab {
    fn default() -> Self {
        ProductionTab {
            selected: WrappingTableState::new(0, enum_iterator::cardinality::<ProductionItem>()),
        }
    }
}

const TABLE_COLS: usize = enum_iterator::cardinality::<Resource>() + 1;
const TABLE_WIDTHS: &[Constraint] = &[Constraint::Ratio(1, TABLE_COLS as u32); TABLE_COLS];

impl Tab for ProductionTab {
    fn draw(&mut self, f: &mut Frame, area: Rect, player: PlayerId, state: &GameState) {
        let player = state.get_player(player);
        let player_stockpile = player.get_stockpile();
        let header =
            Row::new(std::iter::once(Cell::from("Item")).chain(Resource::names().map(Cell::from)));
        let content = enum_iterator::all::<ProductionItem>().map(|item| {
            Row::new(
                std::iter::once(Cell::from(item.to_string())).chain(
                    item.get_cost()
                        .iter()
                        .zip(player_stockpile.iter())
                        .map(|(cost, available)| {
                            Cell::from(cost.to_string() + " / " + &available.to_string())
                        }),
                ),
            )
        });
        let table = Table::new(content, TABLE_WIDTHS.iter())
            .header(header)
            .style(Style::default().fg(Color::White))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Thick)
                    .style(Style::default().bg(Color::DarkGray)),
            )
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol(">>");

        let blocks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(10), Constraint::Percentage(90)].as_ref())
            .split(area);

        let current_production_string = player.get_current_production().map_or(
            "No current production".to_owned(),
            |(item, duration)| {
                "Current production: ".to_owned()
                    + item.to_string().as_str()
                    + ", remaining ticks: "
                    + duration.ticks.to_string().as_str()
            },
        );
        let current_production_display = Paragraph::new(current_production_string).block(
            Block::default()
                .style(Style::default().bg(Color::DarkGray))
                .borders(Borders::ALL)
                .border_type(BorderType::Thick),
        );

        f.render_widget(current_production_display, blocks[0]);
        f.render_stateful_widget(table, blocks[1], self.selected.get_mut());
    }

    fn handle_input(&mut self, player: PlayerId, input: InputAction) -> Option<GameAction> {
        match input {
            InputAction::MoveUp => {
                self.selected.prev();
                None
            }
            InputAction::MoveDown => {
                self.selected.next();
                None
            }
            InputAction::PerformAction => {
                let item =
                    <_ as TryInto<ProductionItem>>::try_into(self.selected.get_row()).unwrap();
                Some(GameAction::Produce(player, item))
            }
            _ => None,
        }
    }
}

const SELL_TABLE_COLS: usize = enum_iterator::cardinality::<Resource>() + 2;
const SELL_TABLE_WIDTHS: &[Constraint] =
    &[Constraint::Ratio(1, SELL_TABLE_COLS as u32); SELL_TABLE_COLS];

struct SellTab {
    selected: WrappingTableState,
}

impl Default for SellTab {
    fn default() -> Self {
        SellTab {
            selected: WrappingTableState::new(0, enum_iterator::cardinality::<SellItem>()),
        }
    }
}

impl Tab for SellTab {
    fn draw(&mut self, f: &mut Frame, area: Rect, player: PlayerId, state: &GameState) {
        let player_stockpile = state.get_player(player).get_stockpile();
        let header = Row::new(
            std::iter::once(Cell::from("Item"))
                .chain(std::iter::once(Cell::from("Value")))
                .chain(Resource::names().map(Cell::from)),
        );
        let content = enum_iterator::all::<SellItem>().map(|item| {
            let Trade {
                give: res,
                receive: money,
            } = state.get_sell_trade(item);
            Row::new(
                std::iter::once(Cell::from(item.to_string()))
                    .chain(std::iter::once(Cell::from(money.to_string())))
                    .chain(
                        res.iter()
                            .zip(player_stockpile.iter())
                            .map(|(cost, available)| {
                                Cell::from(cost.to_string() + " / " + &available.to_string())
                            }),
                    ),
            )
        });
        let table = Table::new(content, SELL_TABLE_WIDTHS.iter())
            .header(header)
            .style(Style::default().fg(Color::White))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Thick)
                    .style(Style::default().bg(Color::DarkGray)),
            )
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol(">>");

        f.render_stateful_widget(table, area, self.selected.get_mut());
    }

    fn handle_input(&mut self, player: PlayerId, input: InputAction) -> Option<GameAction> {
        match input {
            InputAction::MoveUp => {
                self.selected.prev();
                None
            }
            InputAction::MoveDown => {
                self.selected.next();
                None
            }
            InputAction::PerformAction => {
                let item = <_ as TryInto<SellItem>>::try_into(self.selected.get_row()).unwrap();
                Some(GameAction::Sell(player, item))
            }
            _ => None,
        }
    }
}

fn draw_tabs(f: &mut Frame, area: Rect, _: PlayerId, sel: TabType) {
    let tab_bar = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(area);
    let tabs = Tabs::new(
        enum_iterator::all::<TabType>()
            .map(|tab| format!("{} ({})", tab, tab.get_hotkey() as char)),
    )
    .block(Block::default().borders(Borders::ALL))
    .select(sel as usize)
    .highlight_style(
        Style::default()
            .fg(Color::LightRed)
            .add_modifier(Modifier::BOLD),
    );
    f.render_widget(tabs, tab_bar[0]);
}

fn draw_status(f: &mut Frame, area: Rect, _: PlayerId, state: &GameState) {
    let exec_status = if state.is_paused() {
        "Paused"
    } else {
        "Running"
    };
    let exec_status_box = Paragraph::new(exec_status).block(Block::default().borders(Borders::ALL));
    f.render_widget(exec_status_box, area);
}

pub struct Visualization<B: Backend> {
    term: Terminal<B>,
    tab: TabType,
    resource_tab: ResourceTab,
    help_tab: HelpTab,
    prod_tab: ProductionTab,
    sell_tab: SellTab,
}

impl<B: Backend> Visualization<B> {
    pub fn new(term: Terminal<B>) -> Self {
        Visualization {
            term,
            tab: TabType::Resources,
            resource_tab: ResourceTab::default(),
            help_tab: HelpTab::default(),
            prod_tab: ProductionTab::default(),
            sell_tab: SellTab::default(),
        }
    }

    pub fn draw(&mut self, player: PlayerId, state: &mut GameState) {
        let Visualization::<B> {
            term: ref mut t,
            tab: ref mut sel_tab,
            resource_tab: ref mut res_tab,
            help_tab: ref mut h_tab,
            prod_tab: ref mut p_tab,
            sell_tab: ref mut s_tab,
        } = self;
        t.draw(|f| {
            let rects = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Length(f.size().height - 3 * 2 - 2 * 2),
                        Constraint::Length(3),
                    ]
                    .as_ref(),
                )
                .margin(2)
                .split(f.size());
            draw_tabs(f, rects[0], player, *sel_tab);
            match *sel_tab {
                TabType::Resources => res_tab.draw(f, rects[1], player, state),
                TabType::Help => h_tab.draw(f, rects[1], player, state),
                TabType::Production => p_tab.draw(f, rects[1], player, state),
                TabType::Sell => s_tab.draw(f, rects[1], player, state),
            }
            draw_status(f, rects[2], player, state);
        })
        .unwrap();
    }

    pub fn handle_input(&mut self, player: PlayerId, input: InputAction) -> Option<GameAction> {
        let Visualization::<B> {
            tab: ref mut sel_tab,
            resource_tab: ref mut res_tab,
            help_tab: ref mut h_tab,
            prod_tab: ref mut p_tab,
            sell_tab: ref mut s_tab,
            ..
        } = self;
        match input {
            InputAction::TogglePause => Some(GameAction::TogglePause),
            InputAction::SwitchTab(in_tab) => {
                self.tab = in_tab;
                None
            }
            i => match sel_tab {
                TabType::Resources => res_tab.handle_input(player, i),
                TabType::Help => h_tab.handle_input(player, i),
                TabType::Production => p_tab.handle_input(player, i),
                TabType::Sell => s_tab.handle_input(player, i),
            },
        }
    }
}
