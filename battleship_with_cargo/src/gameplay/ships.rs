use crate::gameplay::constants::GRID_SIZE;

use crate::gameplay::gameplay::CellState;

pub fn place_ship(
    grid: &mut [[CellState; GRID_SIZE]; GRID_SIZE], 
    row: usize, 
    col: usize, 
    direction: usize, 
    size: usize,
) -> bool {
    let (dx, dy) = match direction {
        0 => (-1, 0),  // Up
        1 => (0, 1),   // Right
        2 => (1, 0),   // Down
        3 => (0, -1),  // Left
        _ => return false,
    };

    let mut coordinates = Vec::new();
    let mut current_row = row as isize;
    let mut current_col = col as isize;

    for _ in 0..size {
        if current_row < 0 || current_row >= GRID_SIZE as isize || current_col < 0 || current_col >= GRID_SIZE as isize {
            return false;
        }
        if grid[current_row as usize][current_col as usize] != CellState::Empty {
            return false;
        }
        coordinates.push((current_row as usize, current_col as usize));
        current_row += dx;
        current_col += dy;
    }

    for &(r, c) in &coordinates {
        grid[r][c] = CellState::Ship;
    }

    true
}