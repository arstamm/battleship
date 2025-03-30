// Glen Kelley

use std::collections::HashMap;

use crate::gameplay::constants::GRID_SIZE;

use crate::gameplay::gameplay::CellState;

pub fn place_ship(
    grid: &mut [[CellState; GRID_SIZE]; GRID_SIZE], 
    row: usize, 
    col: usize, 
    direction: usize, 
    size: usize,
    cellstate: CellState,
    sprite_map: &mut HashMap<(usize, usize), usize>
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

    for i in 0..size {
        if current_row < 0 || current_row >= GRID_SIZE as isize || current_col < 0 || current_col >= GRID_SIZE as isize {
            return false;
        }
        if grid[current_row as usize][current_col as usize] != CellState::Empty {
            return false;
        }
        coordinates.push((current_row as usize, current_col as usize));
        if i == 0 {
            match direction {
                0 => { sprite_map.insert((row, col), 2); }, // Up
                1 => { sprite_map.insert((row, col), 3); }, // Right
                2 => { sprite_map.insert((row, col), 4); }, // Down
                3 => { sprite_map.insert((row, col), 5); }, // Left
                _ => {}
            }
        } else if i == size - 1 {
            // For the last segment, we can assign a different sprite if needed
            match direction {
                0 => { sprite_map.insert((row, col), 12); }, // Up
                1 => { sprite_map.insert((row, col), 13); }, // Right
                2 => { sprite_map.insert((row, col), 14); }, // Down
                3 => { sprite_map.insert((row, col), 15); }, // Left
                _ => {}
            }
        } else {
            match direction {
                0 => { sprite_map.insert((row, col), 10); }, // Up
                1 => { sprite_map.insert((row, col), 11); }, // Right
                2 => { sprite_map.insert((row, col), 10); }, // Down
                3 => { sprite_map.insert((row, col),11); }, // Left
                _ => {}
            }
        }
        current_row += dx;
        current_col += dy;
    }

    for &(r, c) in &coordinates {
        grid[r][c] = cellstate;
    }

    true
}