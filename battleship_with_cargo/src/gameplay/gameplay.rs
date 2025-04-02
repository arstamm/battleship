use std::collections::HashMap;

// Kelson Gneiting

use ggez::{Context, GameResult};
// use ggez::event::{self, EventHandler, MouseButton};
use ggez::graphics::{self, Canvas, DrawMode, DrawParam, Mesh, MeshBuilder, Rect, Text, TextFragment};
use ggez::mint::{Point2, Vector2};
// use ggez::input::keyboard::{KeyInput, KeyCode};
use ggez::glam::Vec2;

use crate::gameplay::constants;
use crate::gameplay::constants::{GRID_SIZE, CELL_SIZE, X_DELTA, Y_DELTA, BANNER_BACKGROUND_COLOR, BANNER_HEIGHT, BANNER_WIDTH, BANNER_X_POS, BANNER_Y_POS, BUTTON_BACKGROUND_COLOR, BUTTON_HEIGHT, BUTTON_WIDTH, BUTTON_X_POS, BUTTON_Y_POS, FONT_COLOR, X_DELTA_ENEMY};

use crate::gameplay::info::{Player, Banner};


// use crate::gameplay::ships::place_ship;

#[derive(Clone, Copy, PartialEq)]
pub enum CellState {
    Empty,
    Ship,
    Hit,
    Miss,
    Hover,
}

#[derive(Clone, Copy, PartialEq)]
pub enum GameState  {
    Begin,
    ButtonPressedBegin,
    PlayerPlaceShips,
    ButtonPressedPlayerPlaceShips,
    EnemyPlaceShips,
    ButtonPressedEnemyPlaceShips,
    Turns,
    SignalWin,
    Win,
    SignalBegin,
}

pub struct BattleshipGame {
    pub game_state: GameState,
    pub player: Player,
    pub enemy: Player,
    pub text_display_box: Banner,
    pub button: Banner
}

impl BattleshipGame {
    pub fn new() -> Self {
        Self {
            // player_grid: [[CellState::Empty; GRID_SIZE]; GRID_SIZE],
            // enemy_grid: [[CellState::Empty; GRID_SIZE]; GRID_SIZE],
            game_state: GameState::Begin,
            player: Player { 
                x_pos: X_DELTA,
                y_pos: Y_DELTA,
                grid: [[CellState::Empty; GRID_SIZE]; GRID_SIZE],
                sprite_map: HashMap::new(),
                turn: false,
                placing_ships: false,
                start_flag: false,
                end_flag: false,
                current_ship_index: 0,
                current_direction: 1,
                hits: 0,
                num_turns: 0,
                ship_color: constants::SHIP_COLOR,
                grid_cell_color: constants::GRID_CELL_COLOR
            },
            enemy: Player {
                x_pos: X_DELTA_ENEMY,
                y_pos: Y_DELTA,
                grid: [[CellState::Empty; GRID_SIZE]; GRID_SIZE],
                sprite_map: HashMap::new(),
                turn: false,
                placing_ships: false,
                start_flag: false,
                end_flag: false,
                current_ship_index: 0,
                current_direction: 1,
                hits: 0,
                num_turns: 0,
                ship_color: constants::SHIP_COLOR,
                grid_cell_color: constants::GRID_CELL_COLOR
            },
            text_display_box: Banner { x: BANNER_X_POS, 
                y: BANNER_Y_POS, 
                w: BANNER_WIDTH, 
                h: BANNER_HEIGHT, 
                label: "WELCOME TO BATTLESHIP".to_string(),
                background_color: BANNER_BACKGROUND_COLOR,
                font_color: FONT_COLOR
            },
            button: Banner {
                x: BUTTON_X_POS,
                y: BUTTON_Y_POS,
                w: BUTTON_WIDTH,
                h: BUTTON_HEIGHT,
                label: "CLICK\nTO\nBEGIN".to_string(),
                background_color: BUTTON_BACKGROUND_COLOR,
                font_color: FONT_COLOR
            },
        }   
    }

    pub fn display_banner(banner: &Banner, ctx: &mut Context, canvas: & mut Canvas) -> GameResult {
        
        // Create Banner
        let banner_background: Mesh = Mesh::new_rectangle(
            ctx, DrawMode::fill(), 
            Rect { x: banner.x, y: banner.y, w: banner.w, h: banner.h }, 
            banner.background_color
        )?;
        
        let fragment: TextFragment = TextFragment::new(&banner.label)
            .color(banner.font_color)
            .scale(graphics::PxScale::from(constants::FONT_SIZE));

        let text = Text::new(fragment);

        canvas.draw(&banner_background, DrawParam::default());
        canvas.draw(&text, DrawParam::default().dest(Vec2::new( banner.x + constants::TEXT_MARGIN, banner.y + constants::TEXT_MARGIN) ));

        Ok(())
    }

    // Add code to display sprites here. Access sprite_map by using player.sprite_map
    pub fn display_grid(player: &Player, canvas: &mut Canvas, ctx: &mut Context) -> GameResult {

        let mut mesh_builder = MeshBuilder::new();

        for row in 0..GRID_SIZE {
            for col in 0..GRID_SIZE {
                let x = col as f32 * CELL_SIZE + player.x_pos;
                let y = row as f32 * CELL_SIZE + player.y_pos;
                let color = match player.grid[row][col] {
                    CellState::Hit => constants::HIT_COLOR,
                    CellState::Miss => constants::MISS_COLOR,
                    CellState::Hover => constants::HOVER_COLOR,
                    CellState::Ship => player.ship_color,
                    CellState::Empty => player.grid_cell_color
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

                // I would recommend adding code here to start drawing the sprites
                // so they are drawn after each grid block is drawn making it 
                // display on top.
            }
        }
        
        let mesh = Mesh::from_data(ctx, mesh_builder.build());
        canvas.draw(&mesh, graphics::DrawParam::default());

        Ok(())
    }
}

// struct Gameplay {
//     // Grid One

//     // Grid Two

//     //app: GridApp,

// }

// impl Gameplay {
//     // Part One: Place ships
//     pub fn new() {
//         // ?????  
//     }

    

    
//     pub fn start() {

//         let mut quit: bool = false;
//         let mut win: bool = false;

//         loop {
//             // Refresh board and restart game

//             // Player One places ships

//             // Player two places ships

//             // Turns loop
//             loop {
//                 // Handle interaction

//                 // Check for win and then update "win" variable.


//                 if (win) {
//                     break;
//                 }
//                 // Switch Truns
//             }





//             // Play again? (Ask user for input)
//             if (quit) {
//                 break;
//             }

            



//         }

//     }
    

// }