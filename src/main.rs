mod game_of_life;
mod parser;

use game_of_life::{GameOfLife};
use parser::{game_to_debug, debug_to_state};

fn main() {
    let state = debug_to_state(vec!["·····", "·····", "·***·", "·····", "·····"]);

    let mut game = GameOfLife::new(5, 5, &state);

    println!("Before {:?}", game_to_debug(&game));

    game.tick();

    println!("After  {:?}", game_to_debug(&game));
}