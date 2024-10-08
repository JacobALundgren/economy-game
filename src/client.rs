use ratatui::{backend::TermionBackend, Terminal};
use std::{io, io::Read, thread, time::Duration};
use termion::screen::IntoAlternateScreen;
use termion::{async_stdin, input::MouseTerminal, raw::IntoRawMode};

use crate::game_state::GameState;
use crate::input::{parse_input, InputAction};
use crate::player::PlayerId;
use crate::visualization::Visualization;

pub fn run_client(state: &mut GameState, player: PlayerId) {
    let stdout = io::stdout().into_raw_mode().unwrap();
    let stdout = MouseTerminal::from(stdout);
    let stdout = stdout.into_alternate_screen().unwrap();
    let backend = TermionBackend::new(stdout);
    let terminal = Terminal::new(backend).unwrap();
    let mut vis = Visualization::new(terminal);

    let mut stdin = async_stdin().bytes();

    let mut counter = 0;
    'outer: loop {
        while let Some(in_action) = parse_input(&mut stdin) {
            if let Some(game_action) = match in_action {
                InputAction::Quit => break 'outer,
                _ => vis.handle_input(player, in_action),
            } {
                state.handle_action(game_action);
            }
        }
        if !state.is_paused() {
            counter = (counter + 1) % 10;
            if counter == 0 {
                state.step();
            }
        }
        vis.draw(player, state);
        thread::sleep(Duration::from_millis(20));
    }
}
