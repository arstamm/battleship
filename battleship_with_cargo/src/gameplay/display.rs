// Kelson Gneiting

use ggez::{Context, GameResult};
use ggez::event::{EventHandler, MouseButton};
use ggez::graphics;
use ggez::input::keyboard::{KeyInput, KeyCode};

use crate::gameplay::gameplay::BattleshipGame;
use crate::gameplay::eventlisteners;
use crate::gameplay::constants;

use super::gameplay::GameState;

impl EventHandler for BattleshipGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        eventlisteners::check_state(self);
        if self.game_state == GameState::Turns {
            if self.player.turn {
                eventlisteners::check_turn(&mut self.player, &mut self.enemy, &mut self.text_display_box, "--- ENEMY TURN ---", &mut self.button);
            }
            if self.enemy.turn {
                eventlisteners::check_turn(&mut self.enemy, &mut self.player, &mut self.text_display_box, "--- PLAYER TURN ---", &mut self.button); 
            }   
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, constants::BACKGROUND_COLOR);
        
        // Display Grids
        BattleshipGame::display_grid(&self.player, &mut canvas, ctx)?;
        BattleshipGame::display_grid(&self.enemy, &mut canvas, ctx)?;

        // Display Banners
        BattleshipGame::display_banner(&self.text_display_box, ctx, &mut canvas)?;
        BattleshipGame::display_banner(&self.button, ctx, &mut canvas)?;


        canvas.finish(ctx)?;
        Ok(())
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) -> GameResult<()> {
        if button == MouseButton::Left {
            eventlisteners::place_ships(x, y, &mut self.player);
            eventlisteners::place_ships(x, y, &mut self.enemy);
            eventlisteners::click_action(x, y, self);
            eventlisteners::check_guess(x, y, &mut self.player, &mut self.button);
            eventlisteners::check_guess(x, y, &mut self.enemy, &mut self.button);
        }
    
        Ok(())
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, _x: f32, _y: f32, _dx: f32, _dy: f32) -> Result<(), ggez::GameError> {

        eventlisteners::hover_ships(_x, _y, &mut self.player);
        eventlisteners::hover_ships(_x, _y, &mut self.enemy);
        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: KeyInput, _repeat: bool) -> GameResult<()> {
        match keycode.keycode {
            Some(KeyCode::Space) => {
                // self.toggle_direction(&mut self.player); // Toggle direction on spacebar press
                eventlisteners::toggle_direction(&mut self.player);
                eventlisteners::toggle_direction(&mut self.enemy);
            }
            _ => {}
        }
        Ok(())
    }
}


// // Load the sprite sheet once, typically in the game's initialization phase
// let sprite_sheet = graphics::Image::new(ctx, "battleship_with_cargo/src/sprites/Grid_Sprites_3.png")?;

// // Grid layout information
// let sprites_per_row = 6; // Number of sprites in each row
// let sprites_per_column = 4; // Number of sprites in each column
// let sprite_width = 1.0 / sprites_per_row as f32; // Normalized width of each sprite
// let sprite_height = 1.0 / sprites_per_column as f32; // Normalized height of each sprite

// // Define the sprite you want to draw
// Index of the sprite (0-based, left-to-right, top-to-bottom)

// // Calculate the row and column of the sprite
// let sprite_x = (sprite_index % sprites_per_row) as f32; // Column index
// let sprite_y = (sprite_index / sprites_per_row) as f32; // Row index

// // Define the source rectangle for the sprite
// let src_rect = graphics::Rect::new(
//     sprite_x * sprite_width,
//     sprite_y * sprite_height,
//     sprite_width,
//     sprite_height,
// );

// // Draw the sprite instead of a colored rectangle
// canvas.draw(
//     &sprite_sheet,
//     graphics::DrawParam::new()
//         .dest(Point2 { x, y }) // Position on screen
//         .scale(Vector2 { x: CELL_SIZE, y: CELL_SIZE }) // Scale to fit cell size
//         .src(src_rect),
// );
