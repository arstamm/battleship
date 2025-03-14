fn place_ship(grid: &[[u8; GRID_SIZE]; GRID_SIZE], row: u8, col: u8, direction: u8, size: u8) {
    let mut row = row
    let mut col = col
    if direction == 0 {
        let x = -1
        let y = 0
    } else if direction == 1 {
        let x = 0
        let y = 1
    } else if direction == 2 {
        let x = 1
        let y = 0
    } else if direction == 3 {
        let x = 0
        let y = -1
    }

    for i in 0..size {
        grid[row as usize][col as usize] = 1
        row += x;
        col += y;
    }
}