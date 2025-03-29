use crate::gameplay::info::Player;
use crate::gameplay::constants::{CELL_SIZE, GRID_SIZE, SHIP_SIZES};
use crate::gameplay::ships::place_ship;

pub fn toggle_direction(player: &mut Player) {
    player.current_direction = (player.current_direction + 1) % 4; // Cycle through 0-3
}

pub fn place_ships(x: f32, y: f32, p: &mut Player) {
    let col = ((x - p.x_pos) / CELL_SIZE) as usize;
    let row = ((y - p.y_pos) / CELL_SIZE) as usize;

    // println!("Click: x={}, y={}", x, y);
    // println!("Grid Origin: x={}, y={}", p.x_pos, p.y_pos);
    // println!("Calculated: col={}, row={}", col, row);

    if row < GRID_SIZE && col < GRID_SIZE && x - p.x_pos >= 0.0 && y - p.y_pos >= 0.0 {
        if p.placing_ships {
            if p.current_ship_index < SHIP_SIZES.len() {
                if place_ship(&mut p.grid, row, col, p.current_direction, SHIP_SIZES[p.current_ship_index]) {
                    p.current_ship_index += 1;
                    if p.current_ship_index >= SHIP_SIZES.len() {
                        p.placing_ships = false;
                        // player.player_turn = true;
                    }
                }
            }
        }
        // } else if player.turn {
        //     match self.enemy_grid[row][col] {
        //         CellState::Ship => self.enemy_grid[row][col] = CellState::Hit,
        //         CellState::Empty => self.enemy_grid[row][col] = CellState::Miss,
        //         _ => return,
        //     }
        //     self.player_turn = false;
        // }
    }
}