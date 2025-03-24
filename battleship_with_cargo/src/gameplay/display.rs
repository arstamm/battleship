use ggez::{Context, GameResult};
use ggez::event::{self, EventHandler, MouseButton};
use ggez::graphics::{self, Color, DrawMode, MeshBuilder, Text, PxScale, Mesh};
use ggez::mint::{Point2, Vector2};
use ggez::input::keyboard::{KeyInput, KeyCode};

use crate::gameplay::gameplay::BattleshipGame;
use crate::gameplay::constants::GRID_SIZE;
use crate::gameplay::constants::{CELL_SIZE, SHIP_SIZES};
use crate::gameplay::gameplay::CellState;

impl EventHandler for BattleshipGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);
        let mut mesh_builder = MeshBuilder::new();
        
        for row in 0..GRID_SIZE {
            for col in 0..GRID_SIZE {
                let x = col as f32 * CELL_SIZE;
                let y = row as f32 * CELL_SIZE;
                let color = match self.player_grid[row][col] {
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
