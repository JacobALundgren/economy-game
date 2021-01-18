use std::convert::TryInto;
use tui::{backend::Backend, layout::{Constraint, Direction, Layout}, Terminal, widgets::{Block, Borders, Paragraph, TableState}};

use crate::game_state::GameState;
use crate::input::InputAction;
use crate::resource::Resource;

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

pub struct Visualization<B: Backend> {
    term: Terminal<B>,
    selected: WorkerTableState,
}

impl<B: Backend> Visualization<B> {
    pub fn new(term: Terminal<B>) -> Self {
        Visualization {
            term,
            selected: WorkerTableState::new()
        }
    }

    pub fn draw_resources(&mut self, state: &GameState) {
        let Visualization::<B> { term: ref mut t, selected: ref mut sel } = self;
        t.draw(|f| {
            let rects = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(f.size().height - 3 - 2 * 2),
                    Constraint::Length(3),
                ].as_ref())
                .margin(2)
                .split(f.size());
            let top_blocks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Length(rects[0].width - 30),
                    Constraint::Max(30)
                ].as_ref())
                .split(rects[0]);
            let rt = state.resources_as_table();
            f.render_widget(rt, top_blocks[0]);
            let wt = state.player_workers_as_table(0);
            f.render_stateful_widget(wt, top_blocks[1], &mut *sel.get_mut());
            let exec_status = if state.is_paused() {
                "Paused"
            } else {
                "Running"
            };
            let exec_status_box = Paragraph::new(exec_status)
                .block(Block::default().borders(Borders::ALL));
            f.render_widget(exec_status_box, rects[1]);
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
            _ => (),
        }
    }
}
