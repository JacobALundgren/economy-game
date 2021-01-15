mod game_state;
mod player;
mod resource;

use std::{error::Error, io, io::Read, thread, time::Duration, };
use termion::{async_stdin, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen, };
use tui::{backend::TermionBackend, layout::{Constraint, Direction, Layout}, Terminal, widgets::{Block, Borders, Paragraph}, };

use game_state::GameState;

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
    'outer: loop {
        for b in &mut stdin {
            if let Ok(b'q') = b {
                break 'outer;
            } else if let Ok(b'p') = b {
                paused = !paused;
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
            f.render_widget(wt, top_blocks[1]);
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

