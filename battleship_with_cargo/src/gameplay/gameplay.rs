use ggez::{Context, GameResult};
use ggez::event::{self, EventHandler, MouseButton};
use ggez::graphics::{self, Canvas, Color, DrawMode, Mesh, MeshBuilder, PxScale, Text, TextFragment, Rect, DrawParam};
use ggez::mint::{Point2, Vector2};
use ggez::input::keyboard::{KeyInput, KeyCode};
use ggez::glam::Vec2;

use crate::gameplay::constants;
use crate::gameplay::constants::{GRID_SIZE, CELL_SIZE, SHIP_SIZES, X_DELTA, Y_DELTA};
use crate::gameplay::player::Player;

use crate::gameplay::ships::place_ship;

#[derive(Clone, Copy, PartialEq)]
pub enum CellState {
    Empty,
    Ship,
    Hit,
    Miss,
}

pub struct BattleshipGame {
    pub player: Player,
    pub enemy: Player,
    pub banner_text: String,
    pub button_text: String,
}

impl BattleshipGame {
    pub fn new() -> Self {
        Self {
            // player_grid: [[CellState::Empty; GRID_SIZE]; GRID_SIZE],
            // enemy_grid: [[CellState::Empty; GRID_SIZE]; GRID_SIZE],
            player: Player { 
                grid: [[CellState::Empty; GRID_SIZE]; GRID_SIZE],
                turn: true,
                placing_ships: true,
                current_ship_index: 0,
                current_direction: 0
            },
            enemy: Player {
                grid: [[CellState::Empty; GRID_SIZE]; GRID_SIZE],
                turn: false,
                placing_ships: false,
                current_ship_index: 0,
                current_direction: 0
            },
            banner_text: "This is my banner text".to_string(),
            button_text: "Button".to_string(),

        }
    }

    pub fn toggle_direction(&mut self, player: &mut Player) {
        player.current_direction = (player.current_direction + 1) % 4; // Cycle through 0-3
    }


    pub fn place_ships_event_listener(&mut self, x: f32, y: f32, x_pos: f32, y_pos: f32, player: &mut Player) {
        let col = ((x - x_pos) / CELL_SIZE) as usize;
        let row = ((y - y_pos) / CELL_SIZE) as usize;

        if row < GRID_SIZE && col < GRID_SIZE {
            if player.placing_ships {
                if player.current_ship_index < SHIP_SIZES.len() {
                    if place_ship(&mut player.grid, row, col, player.current_direction, SHIP_SIZES[player.current_ship_index]) {
                        player.current_ship_index += 1;
                        if player.current_ship_index >= SHIP_SIZES.len() {
                            player.placing_ships = false;
                            // player.player_turn = true;
                        }
                    }
                }
            }
            
            // else if player.player_turn {
            //     match self.enemy_grid[row][col] {
            //         CellState::Ship => self.enemy_grid[row][col] = CellState::Hit,
            //         CellState::Empty => self.enemy_grid[row][col] = CellState::Miss,
            //         _ => return,
            //     }
            //     self.player_turn = false;
            // }
        }
    }

    pub fn display_banner(x: f32, y: f32, w: f32, h: f32, label: &str, background_color: Color, text_color: Color, ctx: &mut Context, canvas: & mut Canvas) -> GameResult {
        
        // Create Banner
        let banner: Mesh = Mesh::new_rectangle(
            ctx, DrawMode::fill(), 
            Rect { x, y, w, h }, 
            background_color
        )?;
        
        let fragment: TextFragment = TextFragment::new(label)
            .color(text_color)
            .scale(graphics::PxScale::from(constants::FONT_SIZE));

        let text = Text::new(fragment);

        canvas.draw(&banner, DrawParam::default());
        canvas.draw(&text, DrawParam::default().dest(Vec2::new( x + constants::TEXT_MARGIN, y + constants::TEXT_MARGIN) ));

        Ok(())
    }

    pub fn display_grid(player: &Player, x_pos: f32, y_pos: f32, canvas: &mut Canvas, ctx: &mut Context) -> GameResult {

        let mut mesh_builder = MeshBuilder::new();

        for row in 0..GRID_SIZE {
            for col in 0..GRID_SIZE {
                let x = col as f32 * CELL_SIZE + x_pos;
                let y = row as f32 * CELL_SIZE + y_pos;
                let color = match player.grid[row][col] {
                    CellState::Ship => constants::SHIP_COLOR,
                    _ => constants::GRID_CELL_COLOR,
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
                ], 1.0, constants::GRID_LINE_COLOR)?;
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