use std::io;
mod enemy;

fn main() {
    let mut grid: [[i32; 10]; 10] = [[0; 10]; 10];

    // Print the initial grid
    println!("Initial 10x10 Grid:");
    print_grid(&grid);

    // Get user input for row, column, and new value
    let (row, col, value) = get_user_input();

    // Update the grid
    grid[row][col] = value;
    println!("\nUpdated Grid:");
    print_grid(&grid);
}

// Function to print the grid
fn print_grid(grid: &[[i32; 10]; 10]) {
    for row in grid.iter() {
        for &cell in row.iter() {
            print!("{} ", cell);
        }
        println!();
    }
}

// Function to get user input
fn get_user_input() -> (usize, usize, i32) {
    let mut input = String::new();

    println!("\nEnter row (A-J):");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let row = match input.trim().to_uppercase().as_str() {
        "A" => 0,
        "B" => 1,
        "C" => 2,
        "D" => 3,
        "E" => 4,
        "F" => 5,
        "G" => 6,
        "H" => 7,
        "I" => 8,
        "J" => 9,
        _ => {
            println!("Invalid Row");
            return get_user_input();
        }
    };

    input.clear();
    println!("Enter column (1-10):");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let col: usize = input.trim().parse().expect("Invalid number for column");

    input.clear();
    println!("Enter new value:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let value: i32 = input.trim().parse().expect("Invalid number for value");

    (row, col - 1, value)
}