use ggez::{Context, GameResult};
use ggez::event::{self, EventHandler, MouseButton};
use ggez::graphics::{self, Color, DrawMode, DrawParam, Mesh, MeshBuilder, PxScale, Rect, Text};
use ggez::mint::{Point2, Vector2};
use ggez::input::keyboard::{KeyInput, KeyCode};

use crate::gameplay::gameplay::BattleshipGame;
use crate::gameplay::constants::GRID_SIZE;
use crate::gameplay::constants::{CELL_SIZE, SHIP_SIZES};
use crate::gameplay::gameplay::CellState;
use crate::gameplay::constants::{X_DELTA, X_DELTA_ENEMY, Y_DELTA};

impl EventHandler for BattleshipGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);
        
        // Display Grids
        BattleshipGame::display_grid(&self.player_grid, X_DELTA, Y_DELTA, &mut canvas, ctx)?;
        BattleshipGame::display_grid(&self.enemy_grid, X_DELTA_ENEMY, Y_DELTA, &mut canvas, ctx)?;


        let banner: Mesh = Mesh::new_rectangle(
            ctx, DrawMode::fill(), 
            Rect { x: X_DELTA, y: 960.0, w: 1700.0, h: 130.0 }, 
            Color::from_rgb(159, 177, 188) // Cadet Gray
        )?;

        let button: Mesh = Mesh::new_rectangle(
            ctx, DrawMode::fill(), 
            Rect { x: 1860.0, y: 240.0, w: 300.0, h: 240.0 }, 
            Color::from_rgb(110, 136, 152) // Slate Gray
        )?;
        
        let text: Text = Text::new("This is Text");

        canvas.draw(&banner, DrawParam::default());
        canvas.draw(&button, DrawParam::default());
        canvas.draw(&text, DrawParam::default());

        canvas.finish(ctx)?;
        Ok(())
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) -> GameResult<()> {
        if button == MouseButton::Left {
            // self.place_ships_event_listener(x, y);
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
