use crate::gameplay::constants::GRID_SIZE;
use crate::gameplay::constants::CELL_SIZE;
use crate::gameplay::constants::SHIP_SIZES;

use crate::gameplay::ships::place_ship;

#[derive(Clone, Copy, PartialEq)]
pub enum CellState {
    Empty,
    Ship,
    Hit,
    Miss,
}

pub struct BattleshipGame {
    pub player_grid: [[CellState; GRID_SIZE]; GRID_SIZE],
    pub enemy_grid: [[CellState; GRID_SIZE]; GRID_SIZE],
    pub player_turn: bool,
    pub placing_ships: bool,
    pub current_ship_index: usize,
    pub current_direction: usize,
}

impl BattleshipGame {
    pub fn new() -> Self {
        Self {
            player_grid: [[CellState::Empty; GRID_SIZE]; GRID_SIZE],
            enemy_grid: [[CellState::Empty; GRID_SIZE]; GRID_SIZE],
            player_turn: false,
            placing_ships: true,
            current_ship_index: 0,
            current_direction: 1,
        }
    }

    pub fn toggle_direction(&mut self) {
        self.current_direction = (self.current_direction + 1) % 4; // Cycle through 0-3
    }

    pub fn handle_click(&mut self, x: f32, y: f32) {
        let col = (x / CELL_SIZE) as usize;
        let row = (y / CELL_SIZE) as usize;
        if row < GRID_SIZE && col < GRID_SIZE {
            if self.placing_ships {
                if self.current_ship_index < SHIP_SIZES.len() {
                    if place_ship(&mut self.player_grid, row, col, self.current_direction, SHIP_SIZES[self.current_ship_index]) {
                        self.current_ship_index += 1;
                        if self.current_ship_index >= SHIP_SIZES.len() {
                            self.placing_ships = false;
                            self.player_turn = true;
                        }
                    }
                }
            } else if self.player_turn {
                match self.enemy_grid[row][col] {
                    CellState::Ship => self.enemy_grid[row][col] = CellState::Hit,
                    CellState::Empty => self.enemy_grid[row][col] = CellState::Miss,
                    _ => return,
                }
                self.player_turn = false;
            }
        }
    }
}

struct Gameplay {
    // Grid One

    // Grid Two

    //app: GridApp,

}

impl Gameplay {
    // Part One: Place ships
    pub fn new() {

    }

    // Player one Places ships

    // Player two places ships



    // Part Two: While true takes a turn

    // loop {
    //     let (row, col) = app.apply_input();
    //     attempts += 1;

    //     if check_guess_enemy(&app.grid1, row, col) {
    //         println!("Hit! You sunk the battleship in {} attempts!", attempts);
    //         break;
    //     } else {
    //         println!("Miss! Try again.");
    //     }
    // }

    // let (ai_row, ai_col, hit) = ai_attack(&ai_grid);
    // if grid[ai_row][ai_col] == true{
    //     println!("AI hit your ship at row {} and column {}!", ai_row, ai_col);
    // } else {
    //     println!("AI missed at row {} and column {}!", ai_row, ai_col);
    // }

    



    // Play again?
    

}