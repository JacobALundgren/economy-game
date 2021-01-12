mod game_state;
mod player;
mod resource;

use std::{error::Error, io, thread, time};
use termion::{input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Layout},
    Terminal,
};

use game_state::GameState;

fn main() -> Result<(), Box<dyn Error>> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut state = GameState::new(2);
    for _ in 0..10 {
        state.step();
        thread::sleep(time::Duration::from_millis(200));
        terminal.draw(|f| {
            let rects = Layout::default()
                .constraints([Constraint::Percentage(100)].as_ref())
                .margin(5)
                .split(f.size());
            let t = state.as_table();
            f.render_widget(t, rects[0]);
        })?;
    }
    Ok(())
}

