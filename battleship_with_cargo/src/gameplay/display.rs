// Kelson Gneiting

use ggez::{Context, GameResult};
use ggez::event::{self, EventHandler, MouseButton};
use ggez::graphics::{self, Color, DrawMode, DrawParam, Mesh, MeshBuilder, PxScale, Rect, Text, TextFragment, FontData};
use ggez::mint::{Point2, Vector2};
use ggez::input::keyboard::{KeyInput, KeyCode};
use ggez::glam::Vec2;

use crate::gameplay::gameplay::BattleshipGame;
use crate::gameplay::constants::GRID_SIZE;
use crate::gameplay::constants::{CELL_SIZE, SHIP_SIZES};
use crate::gameplay::gameplay::CellState;
use crate::gameplay::constants::{X_DELTA, X_DELTA_ENEMY, Y_DELTA};

use super::constants::{self, BANNER_X_POS, TEXT_MARGIN};

impl EventHandler for BattleshipGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, constants::BACKGROUND_COLOR);
        
        // Display Grids
        BattleshipGame::display_grid(&self.player_grid, X_DELTA, Y_DELTA, &mut canvas, ctx)?;
        BattleshipGame::display_grid(&self.enemy_grid, X_DELTA_ENEMY, Y_DELTA, &mut canvas, ctx)?;

        // Display Banners
        BattleshipGame::display_banner(constants::BANNER_X_POS, constants::BANNER_Y_POS, constants::BANNER_WIDTH, constants::BANNER_HEIGHT, self.banner_text.as_str(), constants::BANNER_BACKGROUND_COLOR,constants::FONT_COLOR, ctx, &mut canvas)?;
        BattleshipGame::display_banner(constants::BUTTON_X_POS, constants::BUTTON_Y_POS, constants::BUTTON_WIDTH, constants::BUTTON_HEIGHT, self.button_text.as_str(), constants::BUTTON_BACKGROUND_COLOR,constants::FONT_COLOR, ctx, &mut canvas)?;

        canvas.finish(ctx)?;
        Ok(())
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) -> GameResult<()> {
        if button == MouseButton::Left {
            // Extract mutable references to avoid borrowing `self` multiple times
            let player_grid = &mut self.player_grid;
            let x_delta = constants::X_DELTA;
            let y_delta = constants::Y_DELTA;

            // Call the event listener with the extracted references
            self.place_ships_event_listener(x, y, x_delta, y_delta, player_grid);
        }
        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: KeyInput, _repeat: bool) -> GameResult<()> {
        match keycode.keycode {
            Some(KeyCode::Space) => {
                self.toggle_direction(); // Toggle direction on spacebar press
            }
            _ => {}
        }
        Ok(())
    }
}
