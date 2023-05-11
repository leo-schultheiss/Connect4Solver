use crate::game_state::Color;

mod game_state;
mod state_space;

// Dimensions
const WIDTH: usize = 7;
const HEIGHT: usize = 6;


fn main() {
    let state = game_state::GameState::new();
    let yellow_win = state.possible_moves()[0].possible_moves()[0].possible_moves()[1].possible_moves()[0].possible_moves()[2].possible_moves()[0].possible_moves()[3];
    println!("{}", yellow_win.straight_winner(Color::RED));
}
