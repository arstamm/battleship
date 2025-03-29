use crate::gameplay::constants::GRID_SIZE;
use crate::gameplay::gameplay::CellState;
use ggez::graphics::Color;

// pub struct player {
//     pub grid: [[CellState; GRID_SIZE]; GRID_SIZE],
//     pub grid: [[CellState; GRID_SIZE]; GRID_SIZE],
//     pub turn: bool,
//     pub ships: bool,
//     pub ship_index: usize,
//     pub current_direction: usize,
// }

pub struct Player {
    pub x_pos: f32,
    pub y_pos: f32,
    pub grid: [[CellState; GRID_SIZE]; GRID_SIZE],
    pub turn: bool,
    pub placing_ships: bool,
    pub current_ship_index: usize,
    pub current_direction: usize,
    // pub current_locations
}

pub struct Banner {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub label: String,
    pub background_color: Color,
    pub font_color: Color
}