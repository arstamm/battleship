use ggez::{Context, GameResult};
use ggez::event::{self, EventHandler, MouseButton};
use ggez::graphics::{self, Canvas, Color, DrawMode, Mesh, MeshBuilder, PxScale, Text};
use ggez::mint::{Point2, Vector2};
use ggez::input::keyboard::{KeyInput, KeyCode};

use crate::gameplay::constants::{GRID_SIZE, CELL_SIZE, SHIP_SIZES, X_DELTA, Y_DELTA};

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

    pub fn place_ships_event_listener(&mut self, x: f32, y: f32, x_pos: f32, y_pos: f32, grid: &mut [[CellState; GRID_SIZE]; GRID_SIZE]) {
        let col = ((x - x_pos) / CELL_SIZE) as usize;
        let row = ((y - y_pos) / CELL_SIZE) as usize;
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

    pub fn display_grid(grid: &[[CellState; GRID_SIZE]; GRID_SIZE], x_pos: f32, y_pos: f32, canvas: &mut Canvas, ctx: &mut Context) -> GameResult {

        let mut mesh_builder = MeshBuilder::new();

        for row in 0..GRID_SIZE {
            for col in 0..GRID_SIZE {
                let x = col as f32 * CELL_SIZE + x_pos;
                let y = row as f32 * CELL_SIZE + y_pos;
                let color = match grid[row][col] {
                    CellState::Ship => Color::GREEN,
                    _ => Color::BLUE,
                };
                
                // Draw cell background
                canvas.draw(
                    &graphics::Quad,
                    graphics::DrawParam::new()
                        .dest(Point2 { x, y })
                        .scale(Vector2 { x: CELL_SIZE, y: CELL_SIZE })
                        .color(color),
                );
                
                // Draw grid outline
                mesh_builder.line(&[
                    Point2 { x, y },
                    Point2 { x: x + CELL_SIZE, y },
                    Point2 { x: x + CELL_SIZE, y: y + CELL_SIZE },
                    Point2 { x, y: y + CELL_SIZE },
                    Point2 { x, y }
                ], 1.0, Color::BLACK)?;
            }
        }
        
        let mesh = Mesh::from_data(ctx, mesh_builder.build());
        canvas.draw(&mesh, graphics::DrawParam::default());

        Ok(())
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
        // ?????  
    }

    

    
    pub fn start() {

        let mut quit: bool = false;
        let mut win: bool = false;

        loop {
            // Refresh board and restart game

            // Player One places ships

            // Player two places ships

            // Turns loop
            loop {
                // Handle interaction

                // Check for win and then update "win" variable.


                if (win) {
                    break;
                }
                // Switch Truns
            }





            // Play again? (Ask user for input)
            if (quit) {
                break;
            }

            



        }

    }
    

}