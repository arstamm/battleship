// luke hill
use rand::Rng;
// use std::io;
use crate::gameplay::GridApp;

const GRID_SIZE: usize = 10;

pub fn main_enemy() {
    let app = GridApp::new(); // Create an instance of GridApp
    // let mut grid = create_grid();
    let mut ai_grid: [[bool; GRID_SIZE]; GRID_SIZE] = [[false; GRID_SIZE]; GRID_SIZE]; // Track AI attacks
    place_ship_enemy(&app.grid1);

    println!("Welcome to Battleship!");
    
    let mut attempts: u32 = 0;
    loop {
        let (row, col) = app.apply_input();
        attempts += 1;

        if check_guess_enemy(&app.grid1, row, col) {
            println!("Hit! You sunk the battleship in {} attempts!", attempts);
            break;
        } else {
            println!("Miss! Try again.");
        }
    }

    let (ai_row, ai_col, hit) = ai_attack(&ai_grid);
    if grid[ai_row][ai_col] == true{
        println!("AI hit your ship at row {} and column {}!", ai_row, ai_col);
    } else {
        println!("AI missed at row {} and column {}!", ai_row, ai_col);
    }
}

// pub fn create_grid() -> [[usize; GRID_SIZE]; GRID_SIZE] {
//     [[0; GRID_SIZE]; GRID_SIZE]
// }

pub fn place_ship_enemy(grid: &[[u8; GRID_SIZE]; GRID_SIZE]) {
    let mut rng = rand::thread_rng();
    let row = rng.gen_range(0..GRID_SIZE);
    let col = rng.gen_range(0..GRID_SIZE);
    grid[row][col] = 1;
}

pub fn check_guess_enemy(grid: &[[u8; GRID_SIZE]; GRID_SIZE], row: u8, col: u8) -> bool {
    app::grid2[row][col] == 1;
    if grid: app::grid2[row][col] == 0 {
        grid: app::grid2[row][col] += 2; // Mark as missed
        false
    } 
    else if grid: app::grid2[row][col] == 1 {
        grid: app::grid2[row][col] += 2; // Mark as hit
        true
    }
}

fn ai_attack(grid: &[[bool; GRID_SIZE]; GRID_SIZE]) -> (usize, usize, bool) {
    let mut rng = rand::thread_rng();
    let (mut row, mut col);

    // Ensure AI does not attack the same spot twice
    loop {
        row = rng.gen_range(0..GRID_SIZE);
        col = rng.gen_range(0..GRID_SIZE);

        if !grid: app::grid2[row][col] {
            grid: app::grid2[row][col] > 1; // Mark as attacked
            break;
        }
    }

    println!("AI attacks row {} and column {}!", row, col);
    (row, col, grid: app::grid2[row][col]) // Return attack position and result
}