use ggez::{Context, GameResult};
use ggez::event::{self, EventHandler, MouseButton};
use ggez::graphics::{self, Color, DrawMode, MeshBuilder, Text, PxScale, Mesh};
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
        let mut mesh_builder = MeshBuilder::new();
        
        // Display Grids
        BattleshipGame::display_grid(&self.player_grid, X_DELTA, Y_DELTA, &mut mesh_builder, &mut canvas, ctx)?;
        BattleshipGame::display_grid(&self.enemy_grid, X_DELTA_ENEMY, Y_DELTA, &mut mesh_builder, &mut canvas, ctx)?;
        
        let mesh = Mesh::from_data(ctx, mesh_builder.build());
        canvas.draw(&mesh, graphics::DrawParam::default());

        canvas.finish(ctx)?;
        Ok(())
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) -> GameResult<()> {
        if button == MouseButton::Left {
            self.handle_click(x, y);
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
