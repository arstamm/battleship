const GRID_SIZE: usize = 10;

fn place_ship(
    grid: &mut [[u8; GRID_SIZE]; GRID_SIZE], row: u8, col: u8, direction: u8, size: u8,){
    // Define the direction vectors
    let (dx, dy) = match direction {
        0 => (-1, 0),  // Up
        1 => (0, 1),   // Right
        2 => (1, 0),   // Down
        3 => (0, -1),  // Left
        _ => return Err("Invalid direction"), // Handle invalid direction
    };

    // Collect the coordinates
    let mut coordinates = Vec::new();
    let mut current_row = row as i32;
    let mut current_col = col as i32;

    for _ in 0..size {
        // Check if the coordinates are within bounds
        if current_row < 0 || current_row >= GRID_SIZE as i32 || current_col < 0 || current_col >= GRID_SIZE as i32 {
            return Err("Ship placement out of bounds");
        }
        // Check if the cell is empty
        if grid[current_row as usize][current_col as usize] != 0 {
            return Err("Ship placement overlaps with another ship");
        }
        coordinates.push((current_row as usize, current_col as usize));

        // Update the coordinates
        current_row += dx;
        current_col += dy;
    }

    // Place the ship on the grid
    for &(r, c) in &coordinates {
        grid[r][c] = 1;
    }
}
