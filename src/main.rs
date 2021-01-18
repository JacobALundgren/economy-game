mod game_state;
mod input;
mod player;
mod resource;

use std::{convert::TryInto, error::Error, io, io::Read, thread, time::Duration, };
use termion::{async_stdin, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen, };
use tui::{backend::TermionBackend, layout::{Constraint, Direction, Layout}, Terminal, widgets::{Block, Borders, Paragraph, TableState}, };

use game_state::GameState;
use input::{InputAction, parse_input};
use resource::Resource;

#[derive(Default)]
struct WorkerTableState {
    state : TableState,
}

impl WorkerTableState {
    fn get(&mut self) -> &mut TableState {
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

fn main() -> Result<(), Box<dyn Error>> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut stdin = async_stdin().bytes();

    let mut state = GameState::new(2);
    let mut paused = false;
    let mut counter = 0;
    let mut selected = WorkerTableState::new();
    'outer: loop {
        while let Some(in_action) = parse_input(&mut stdin) {
            match in_action {
                InputAction::Quit => break 'outer,
                InputAction::TogglePause => paused = !paused,
                InputAction::MoveUp => selected.prev(),
                InputAction::MoveDown => selected.next(),
                InputAction::Decrease => {
                    let resource = <_ as TryInto<Resource>>::try_into(selected.get_row() - 1).unwrap();
                    state.deallocate_player_worker(0, resource);
                },
                InputAction::Increase => {
                    let resource = <_ as TryInto<Resource>>::try_into(selected.get_row() - 1).unwrap();
                    state.allocate_player_worker(0, resource);
                },
            }
        }
        if !paused {
            counter = (counter + 1) % 10;
            if counter == 0 {
                state.step();
            }
        }
        terminal.draw(|f| {
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
            f.render_stateful_widget(wt, top_blocks[1], selected.get());
            let exec_status = if paused {
                "Paused"
            } else {
                "Running"
            };
            let exec_status_box = Paragraph::new(exec_status)
                .block(Block::default().borders(Borders::ALL));
            f.render_widget(exec_status_box, rects[1]);
        })?;
        thread::sleep(Duration::from_millis(20));
    }
    Ok(())
}

