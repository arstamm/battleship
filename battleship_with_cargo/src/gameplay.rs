use crate::constants::GRID_SIZE;

pub enum FocusedField {
    Row,
    Column,
    Value,
}

pub struct GridApp {
    pub grid1: [[u8; GRID_SIZE]; GRID_SIZE],
    pub grid2: [[u8; GRID_SIZE]; GRID_SIZE],
    pub input_mode: bool,
    pub row_input: String,
    pub col_input: String,
    pub value_input: String,
    pub focused_field: FocusedField,
}

impl GridApp {
    pub fn new() -> Self {
        GridApp {
            grid1: [[0; GRID_SIZE]; GRID_SIZE],
            grid2: [[0; GRID_SIZE]; GRID_SIZE],
            input_mode: false,
            row_input: String::new(),
            col_input: String::new(),
            value_input: String::new(),
            focused_field: FocusedField::Row,
        }
    }

    // pub fn update(&mut self, ctx: &mut Context) -> GameResult {
    //     handle_input(self, ctx)?;
    //     Ok(())
    // }

    pub fn reset(&mut self) {
        self.grid1 = [[0; GRID_SIZE]; GRID_SIZE];
        self.grid2 = [[0; GRID_SIZE]; GRID_SIZE];
        self.input_mode = false;
        self.row_input.clear();
        self.col_input.clear();
        self.value_input.clear();
    }

    pub fn apply_input(&mut self) -> (u8, u8) {
        let row = match self.row_input.trim().to_uppercase().as_str() {
            "A" => 0, "B" => 1, "C" => 2, "D" => 3, "E" => 4,
            "F" => 5, "G" => 6, "H" => 7, "I" => 8, "J" => 9,
            _ => {
                println!("Invalid row: {}", self.row_input);
                return (0, 0); // Return a default value or handle the error appropriately
            }
        };
    
        let col: u8 = match self.col_input.trim().parse() {
            Ok(n) if (1..=GRID_SIZE).contains(&n) => (n - 1) as u8,
            _ => {
                println!("Invalid column: {}", self.col_input);
                return (0, 0); // Return a default value or handle the error appropriately
            }
        };
    
        (row, col)
    }
    
    // pub fn apply_input(&mut self) {
    //     let row = match self.row_input.trim().to_uppercase().as_str() {
    //         "A" => 0, "B" => 1, "C" => 2, "D" => 3, "E" => 4,
    //         "F" => 5, "G" => 6, "H" => 7, "I" => 8, "J" => 9,
    //         _ => {
    //             println!("Invalid row: {}", self.row_input);
    //             return;
    //         }
    //     };
    
    //     let col: usize = match self.col_input.trim().parse() {
    //         Ok(n) if (1..=GRID_SIZE).contains(&n) => (n - 1) as usize,
    //         _ => {
    //             println!("Invalid column: {}", self.col_input);
    //             return;
    //         }
    //     };
    
    //     let value: usize = match self.value_input.trim().parse() {
    //         Ok(v) => v,
    //         _ => {
    //             println!("Invalid value: {}", self.value_input);
    //             return;
    //         }
    //     };
    
    //     self.grid1[row][col] = value;
    //     println!("Grid 1: Set value {} at row {}, column {}", value, row, col);
    
    //     self.close_input_dialog();
    // }

    pub fn open_input_dialog(&mut self) {
        self.input_mode = true;
    }

    pub fn close_input_dialog(&mut self) {
        self.input_mode = false;
        self.row_input.clear();
        self.col_input.clear();
        self.value_input.clear();
    }
}

struct Gameplay {
    // Grid One

    // Grid Two

}

impl Gameplay {
    // Part One: Place ships



    // Part Two: While true takes a turn



    // Play again?
    

}