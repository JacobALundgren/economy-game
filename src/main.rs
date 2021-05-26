mod client;
mod game_state;
mod input;
mod player;
mod production;
mod resource;
mod sell;
mod visualization;

use client::run_client;
use game_state::GameState;

fn main() {
    let mut state = GameState::new();
    state.register_player();
    state.register_player();

    run_client(&mut state, 0);
}
