use rand::Rng;
use std::io;

const GRID_SIZE: usize = 10;

fn main() {
    let mut grid = create_grid();
    place_ship(&mut grid);

    println!("Welcome to Battleship!");
    
    let mut attempts = 0;
    loop {
        let (row, col) = get_user_input();
        attempts += 1;

        if check_guess(&grid, row, col) {
            println!("Hit! You sunk the battleship in {} attempts!", attempts);
            break;
        } else {
            println!("Miss! Try again.");
        }
    }
}

fn create_grid() -> [[u8; GRID_SIZE]; GRID_SIZE] {
    [[0; GRID_SIZE]; GRID_SIZE]
}

fn place_ship(grid: &mut [[u8; GRID_SIZE]; GRID_SIZE]) {
    let mut rng = rand::thread_rng();
    let row = rng.gen_range(0..GRID_SIZE);
    let col = rng.gen_range(0..GRID_SIZE);
    grid[row][col] = 1;
}

fn check_guess(grid: &[[u8; GRID_SIZE]; GRID_SIZE], row: usize, col: usize) -> bool {
    grid[row][col] == 1
}