use crate::game_of_life::{State, GameOfLife, Cell};

pub fn debug_to_state(debug: Vec<&str>) -> State {
    debug
        .join("")
        .chars()
        .map(|value| {
            if value == '*' { Cell::Alive } else { Cell::Dead }
        })
        .collect()
}

pub fn game_to_debug(game: &GameOfLife) -> Vec<String> {
    game.state
        .chunks(game.dimensions.width)
        .map(|row| {
            row.to_vec().iter().map(|cell| {
                if *cell == Cell::Alive { "*" } else { "·" }
            }).collect::<Vec<&str>>().join("")
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_a_debug_to_a_state() {
        let given = vec!["*··", "·*·", "··*"];
        let expected = [
            Cell::Alive, Cell::Dead, Cell::Dead,
            Cell::Dead, Cell::Alive, Cell::Dead,
            Cell::Dead, Cell::Dead, Cell::Alive];

        let result = debug_to_state(given);

        assert_eq!(result, expected);
    }

    #[test]
    fn it_parses_a_game_to_a_debug() {
        let given = GameOfLife::new(3, 3, &vec![
            Cell::Alive, Cell::Dead, Cell::Dead,
            Cell::Dead, Cell::Alive, Cell::Dead,
            Cell::Dead, Cell::Dead, Cell::Alive]);
        let expected = vec!["*··", "·*·", "··*"];

        let result = game_to_debug(&given);

        assert_eq!(result, expected);
    }
}