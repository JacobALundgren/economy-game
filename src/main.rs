mod game_state;
mod input;
mod player;
mod resource;
mod visualization;

use std::{error::Error, io, io::Read, thread, time::Duration, };
use termion::{async_stdin, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen, };
use tui::{backend::TermionBackend, Terminal, };

use game_state::GameState;
use input::{InputAction, parse_input};
use visualization::Visualization;

fn main() -> Result<(), Box<dyn Error>> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    let mut vis = Visualization::new(terminal);

    let mut stdin = async_stdin().bytes();

    let mut state = GameState::new(2);
    let mut counter = 0;
    'outer: loop {
        while let Some(in_action) = parse_input(&mut stdin) {
            match in_action {
                InputAction::Quit => break 'outer,
                _ => vis.handle_input(in_action, &mut state),
            }
        }
        if !state.is_paused() {
            counter = (counter + 1) % 10;
            if counter == 0 {
                state.step();
            }
        }
        vis.draw(&mut state);
        thread::sleep(Duration::from_millis(20));
    }
    Ok(())
}

