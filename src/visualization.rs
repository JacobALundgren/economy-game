use std::{fmt, convert::{TryFrom, TryInto}};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    terminal::{Frame, Terminal},
    text::{Span, Spans},
    widgets::{Block, Borders, BorderType, Cell, Paragraph, Row, Table, TableState, Tabs}
};

use crate::game_state::GameState;
use crate::input::InputAction;
use crate::resource::Resource;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Tab {
    Resources = 0,
    Help = 1,
}

impl Tab {
    pub const fn count() -> usize {
        (Tab::Help as usize) + 1
    }

    pub fn get_hotkey(&self) -> u8 {
        match self {
            Tab::Resources => 'r' as u8,
            Tab::Help => 'h' as u8,
        }
    }
}

impl TryFrom<usize> for Tab {
    type Error = ();

    fn try_from(v: usize) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Tab::Resources),
            1 => Ok(Tab::Help),
            _ => Err(()),
        }
    }
}

impl fmt::Display for Tab {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Default)]
struct WorkerTableState {
    state : TableState,
}

impl WorkerTableState {
    fn get_mut(&mut self) -> &mut TableState {
        &mut self.state
    }

    fn get_row(&self) -> usize {
        self.state.selected().unwrap()
    }

    fn new() -> Self {
        let mut ret = WorkerTableState::default();
        ret.state.select(Some(1));
        ret
    }

    fn next(&mut self) {
        let curr = self.state.selected().unwrap();
        self.state.select(Some(curr.rem_euclid(Resource::count()) + 1));
    }

    fn prev(&mut self) {
        let curr = self.state.selected().unwrap();
        self.state.select(Some(((curr as i32 - 2).rem_euclid(Resource::count() as i32) + 1) as usize));
    }
}

fn draw_tabs<B: Backend>(f: &mut Frame<B>, area: Rect, sel: Tab) {
    let tab_bar = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(100)
        ].as_ref())
        .split(area);
    let tabs = Tabs::new(
            (0..Tab::count())
                .into_iter()
                .map(|i| <_ as TryInto<Tab>>::try_into(i).unwrap())
                .map(|tab| Spans::from(vec![Span::from(format!("{} ({})", tab.to_string(), tab.get_hotkey() as char))])).collect()
        )
        .block(Block::default().borders(Borders::ALL))
        .select(sel as usize)
        .highlight_style(Style::default()
            .fg(Color::LightRed)
            .add_modifier(Modifier::BOLD));
    f.render_widget(tabs, tab_bar[0]);
}

fn draw_resources<B: Backend>(f: &mut Frame<B>, area: Rect, state: &GameState, selected: &mut WorkerTableState) {
    let main_blocks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(area.width - 30),
            Constraint::Max(30)
        ].as_ref())
        .split(area);
    let rt = state.resources_as_table();
    f.render_widget(rt, main_blocks[0]);
    let wt = state.player_workers_as_table(0);
    f.render_stateful_widget(wt, main_blocks[1], selected.get_mut());
}

fn draw_help<B: Backend>(f: &mut Frame<B>, area: Rect) {
    let blocks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(80),
            Constraint::Percentage(20)
        ].as_ref())
        .split(area);
    let overview = Paragraph::new(
        concat!("Control the allocation of your workers to different resources",
            " using the arrow keys. Balance your economy to produce what you need."))
        .block(Block::default()
            .style(Style::default()
                .bg(Color::DarkGray))
            .borders(Borders::ALL)
            .border_type(BorderType::Thick)
            .title(Span::from("Overview")));
    f.render_widget(overview, blocks[0]);

    let hotkeys = vec![
        ("p", "Toggle pause"),
        ("q", "Exit program")
    ];
    let table = Table::new(hotkeys
            .iter()
            .map(|(key, desc)| Row::new(vec![Cell::from(*key), Cell::from(*desc)])))
        .widths(&[Constraint::Percentage(10), Constraint::Percentage(90)])
        .block(Block::default()
            .style(Style::default()
                .bg(Color::DarkGray))
            .borders(Borders::ALL)
            .border_type(BorderType::Thick)
            .title(Span::from("Hotkeys")));
    f.render_widget(table, blocks[1]);
}

fn draw_status<B: Backend>(f: &mut Frame<B>, area: Rect, state: &GameState) {
    let exec_status = if state.is_paused() {
        "Paused"
    } else {
        "Running"
    };
    let exec_status_box = Paragraph::new(exec_status)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(exec_status_box, area);
}

pub struct Visualization<B: Backend> {
    term: Terminal<B>,
    selected: WorkerTableState,
    tab: Tab,
}

impl<B: Backend> Visualization<B> {
    pub fn new(term: Terminal<B>) -> Self {
        Visualization {
            term,
            selected: WorkerTableState::new(),
            tab: Tab::Resources,
        }
    }


    pub fn draw(&mut self, state: &mut GameState) {
        let Visualization::<B> { term: ref mut t, selected: ref mut sel, tab: ref mut sel_tab } = self;
        t.draw(|f| {
            let rects = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Length(f.size().height - 3 * 2 - 2 * 2),
                    Constraint::Length(3),
                ].as_ref())
                .margin(2)
                .split(f.size());
            draw_tabs(f, rects[0], *sel_tab);
            match *sel_tab {
                Tab::Resources => draw_resources(f, rects[1], state, sel),
                Tab::Help => draw_help(f, rects[1]),
            }
            draw_status(f, rects[2], &state);
        }).unwrap();

    }

    pub fn handle_input(&mut self, input: InputAction, state: &mut GameState) {

        match input {
            InputAction::TogglePause => state.toggle_paused(),
            InputAction::MoveUp => self.selected.prev(),
            InputAction::MoveDown => self.selected.next(),
            InputAction::Decrease => {
                let resource = <_ as TryInto<Resource>>::try_into(self.selected.get_row() - 1).unwrap();
                state.deallocate_player_worker(0, resource);
            },
            InputAction::Increase => {
                let resource = <_ as TryInto<Resource>>::try_into(self.selected.get_row() - 1).unwrap();
                state.allocate_player_worker(0, resource);
            },
            InputAction::SwitchTab(in_tab) => self.tab = in_tab,
            _ => (),
        }
    }
}
