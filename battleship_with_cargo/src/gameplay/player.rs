use crate::gameplay::constants::{GRID_SIZE, CELL_SIZE, SHIP_SIZES, X_DELTA, Y_DELTA};
use crate::gameplay::gameplay::CellState;

// pub struct player {
//     pub grid: [[CellState; GRID_SIZE]; GRID_SIZE],
//     pub grid: [[CellState; GRID_SIZE]; GRID_SIZE],
//     pub turn: bool,
//     pub ships: bool,
//     pub ship_index: usize,
//     pub current_direction: usize,
// }

pub struct Player {
    pub grid: [[CellState; GRID_SIZE]; GRID_SIZE],
    pub turn: bool,

    pub placing_ships: bool,
    pub current_ship_index: usize,
    pub current_direction: usize,
    // pub current_locations
}