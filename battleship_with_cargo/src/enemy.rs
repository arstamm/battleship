use rand::Rng;
use std::io;
mod main::apply_input;

const GRID_SIZE: usize = 10;

pub fn main_enemy() {
    let mut grid = create_grid();
    place_ship(&mut grid);

    println!("Welcome to Battleship!");
    
    let mut attempts = 0;
    loop {
        let (row, col) = apply_input();
        attempts += 1;

        if check_guess(&grid, row, col) {
            println!("Hit! You sunk the battleship in {} attempts!", attempts);
            break;
        } else {
            println!("Miss! Try again.");
        }
    }
}

pub fn create_grid() -> [[u8; GRID_SIZE]; GRID_SIZE] {
    [[0; GRID_SIZE]; GRID_SIZE];
}

pub fn place_ship_enemy(grid: &mut [[u8; GRID_SIZE]; GRID_SIZE]) {
    let mut rng = rand::thread_rng();
    let row = rng.gen_range(0..GRID_SIZE);
    let col = rng.gen_range(0..GRID_SIZE);
    grid[row][col] = 1;
}

pub fn check_guess_enemy(grid: &[[u8; GRID_SIZE]; GRID_SIZE], row: usize, col: usize) -> bool {
    grid[row][col] == 1;
    if grid[row][col] == 0 {
        println!("Miss!");
        grid[row][col] += 2; // Mark as missed
    } 
    else if grid[row][col] == 1 {
        println!("Hit!");
        grid[row][col] += 2; // Mark as hit
    }
}
