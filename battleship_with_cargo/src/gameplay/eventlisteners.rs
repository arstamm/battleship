use crate::gameplay::info::Player;
use crate::gameplay::constants::{CELL_SIZE, GRID_SIZE, SHIP_SIZES, SHIP_COLOR, HOVER_COLOR};
use crate::gameplay::gameplay::CellState;
use crate::gameplay::ships::place_ship;
use crate::gameplay::info::Banner;

pub fn toggle_direction(player: &mut Player) {
    player.current_direction = (player.current_direction + 1) % 4; // Cycle through 0-3
}

pub fn place_ships(x: f32, y: f32, p: &mut Player) {
    let col = ((x - p.x_pos) / CELL_SIZE) as usize;
    let row = ((y - p.y_pos) / CELL_SIZE) as usize;

    // println!("Click: x={}, y={}", x, y);
    // println!("Grid Origin: x={}, y={}", p.x_pos, p.y_pos);
    // println!("Calculated: col={}, row={}", col, row);

            // Reset Image
        for r in 0..GRID_SIZE {
            for c in 0..GRID_SIZE {
                if p.grid[r][c] == CellState::Hover {
                    p.grid[r][c] = CellState::Empty;
                }
            }
        }

    if row < GRID_SIZE && col < GRID_SIZE && x - p.x_pos >= 0.0 && y - p.y_pos >= 0.0 {
        if p.placing_ships {
            if p.current_ship_index < SHIP_SIZES.len() {
                if place_ship(&mut p.grid, row, col, p.current_direction, SHIP_SIZES[p.current_ship_index], CellState::Ship) {
                    p.current_ship_index += 1;
                    if p.current_ship_index >= SHIP_SIZES.len() {
                        p.placing_ships = false;
                    }
                }
            }
        }
    }
}

pub fn click_action(x: f32, y: f32, banner: &mut Banner, player: &mut Player) {
     
    if x >= banner.x && y >= banner.y && x <= banner.x + banner.w && y <= banner.y + banner.h {
        println!("The button was pressed!");
        if player.placing_ships == false {
            println!("(Ships Hidden)");
            player.ship_color = player.grid_cell_color;
            player.turn = true;

        }
    }
}

pub fn check_guess(x: f32, y: f32, p: &mut Player) {
    let col = ((x - p.x_pos) / CELL_SIZE) as usize;
    let row = ((y - p.y_pos) / CELL_SIZE) as usize;

    if row < GRID_SIZE && col < GRID_SIZE && x - p.x_pos >= 0.0 && y - p.y_pos >= 0.0 {
        if p.turn {
            if p.grid[row][col] == CellState::Ship {
                p.grid[row][col] = CellState::Hit;
                p.hits += 1;
            } else if p.grid[row][col] == CellState::Empty {
                p.grid[row][col] = CellState::Miss;
            }
        }
    }
}

pub fn hover_ships(x: f32, y: f32, p: &mut Player) {
    let col = ((x - p.x_pos) / CELL_SIZE) as usize;
    let row = ((y - p.y_pos) / CELL_SIZE) as usize;

    // println!("Click: x={}, y={}", x, y);
    // println!("Grid Origin: x={}, y={}", p.x_pos, p.y_pos);
    // println!("Calculated: col={}, row={}", col, row);
    if p.placing_ships {

        // Reset Image
        for r in 0..GRID_SIZE {
            for c in 0..GRID_SIZE {
                if p.grid[r][c] == CellState::Hover {
                    p.grid[r][c] = CellState::Empty;
                }
            }
        }

        // Create Hover Effect
        if row < GRID_SIZE && col < GRID_SIZE && x - p.x_pos >= 0.0 && y - p.y_pos >= 0.0 {
            if p.current_ship_index < SHIP_SIZES.len() {
                place_ship(&mut p.grid, row, col, p.current_direction, SHIP_SIZES[p.current_ship_index], CellState::Hover);
            }
        }
    }
}



pub fn check_win(p: &mut Player) {
    if p.turn {
        if p.hits >= 17 {
            p.ship_color = SHIP_COLOR;
            p.grid_cell_color = HOVER_COLOR;
            for r in 0..GRID_SIZE {
                for c in 0..GRID_SIZE {
                    if p.grid[r][c] == CellState::Hit {
                        p.grid[r][c] = CellState::Ship;
                    }
                }
            }
            p.turn = false;
        }
    }
}