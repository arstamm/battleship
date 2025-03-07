// fn main() {
//     println!("Hello, world!");
//     // Do next: Figure out how to connect multiple files, and display screen with 4 10 by 10 grids
// }

use std::io;

use ggez::event;
use ggez::graphics::{self, Color};
use ggez::{Context, GameResult};
use ggez::glam::*;

struct MainState {
    pos_x: f32,
}

impl MainState {
    fn new() -> GameResult<MainState> {
        let s = MainState { pos_x: 0.0 };
        Ok(s)
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.pos_x = self.pos_x % 800.0 + 1.0;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(
            ctx,
            graphics::Color::from([0.0, 0.0, 1.0, 1.0]),
        );

        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            Vec2::new(0.0, 0.0),
            100.0,
            2.0,
            Color::RED,
        )?;
        canvas.draw(&circle, Vec2::new(self.pos_x, 380.0));

        canvas.finish(ctx)?;
        Ok(())
    }
}

pub fn main() -> GameResult {

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

    let cb = ggez::ContextBuilder::new("super_simple", "ggez");
    let (ctx, event_loop) = cb.build()?;
    let state = MainState::new()?;
    event::run(ctx, event_loop, state)
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

