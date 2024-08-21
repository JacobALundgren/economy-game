mod client;
mod game_state;
mod input;
mod player;
mod production;
mod resource;
mod sell;
mod visualization;

use client::run_client;
use game_state::GameAction;
use game_state::GameState;
use resource::Resource;

fn main() {
    let mut state = GameState::new();
    state.register_player();
    state.register_player();

    for res in enum_iterator::all::<Resource>() {
        state.handle_action(GameAction::AllocateWorker(1, res));
    }

    run_client(&mut state, 0);
}
