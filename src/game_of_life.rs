#[derive(PartialEq, Debug, Clone)]
pub enum Cell {
    Alive,
    Dead,
}

pub type State = Vec<Cell>;

pub struct Dimensions { pub width: usize, pub height: usize }

pub struct GameOfLife {
    pub dimensions: Dimensions,
    pub state: State,
}

impl GameOfLife {
    pub fn new(width: usize, height: usize, initial: &State) -> GameOfLife {
        if initial.len() != width * height {
            panic!("Invalid initial state");
        }

        GameOfLife {
            dimensions: Dimensions { width, height },
            state: initial.clone(),
        }
    }

    pub fn tick(&mut self) {
        let new_state = self.state.iter().enumerate().map(|(index, cell)| {
            let alive_neighbors = self.alive_neighbors(&index);

            let new_cell: Cell = match (cell, alive_neighbors) {
                (Cell::Alive, x) if x < 2 => Cell::Dead,
                (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                (Cell::Alive, x)if x > 3 => Cell::Dead,
                (Cell::Dead, 3) => Cell::Alive,
                _ => Cell::Dead
            };

            new_cell
        }).collect();

        self.state = new_state;
    }

    fn alive_neighbors(&self, index: &usize) -> u32 {
        let row = index / self.dimensions.width;
        let column = index - row * self.dimensions.width;
        let mut count: u32 = 0;

        for delta_row in [self.dimensions.height - 1, 0, 1].iter() {
            for delta_column in [self.dimensions.width - 1, 0, 1].iter() {
                if *delta_row == 0 && *delta_column == 0 {
                    continue;
                }

                let neighbor_row = (row + *delta_row) % self.dimensions.height;
                let neighbor_column = (column + *delta_column) % self.dimensions.width;
                let neighbor_index = neighbor_row * self.dimensions.width + neighbor_column;

                count += if self.state[neighbor_index] == Cell::Alive { 1 } else { 0 };
            }
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::debug_to_state;

    #[test]
    fn block() {
        let block = debug_to_state(vec!["····", "·**·", "·**·", "····"]);
        let mut game = GameOfLife::new(4, 4, &block);

        game.tick();

        assert_eq!(game.state, block);
    }

    #[test]
    fn beacon() {
        let beacon = debug_to_state(vec!["······", "·**···", "·*····", "····*·", "···**·", "······"]);
        let mut game = GameOfLife::new(6, 6, &beacon);

        game.tick();

        assert_eq!(game.state, debug_to_state(vec![
            "······",
            "·**···",
            "·**···",
            "···**·",
            "···**·",
            "······",
        ]));
    }

    #[test]
    fn glider() {
        let beacon = debug_to_state(vec![
            "··········",
            "·····*····",
            "···*·*····",
            "····**····",
            "··········", ]);
        let mut game = GameOfLife::new(10, 5, &beacon);

        game.tick();

        assert_eq!(game.state, debug_to_state(vec![
            "··········",
            "····*·····",
            "·····**···",
            "····**····",
            "··········",
        ]));
    }
}