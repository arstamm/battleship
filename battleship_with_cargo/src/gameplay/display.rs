use ggez::{Context, GameResult};
use ggez::event::{EventHandler, MouseButton};
use ggez::graphics;
use ggez::input::keyboard::{KeyInput, KeyCode};

use crate::gameplay::gameplay::BattleshipGame;
use crate::gameplay::eventlisteners;
use crate::gameplay::constants;

impl EventHandler for BattleshipGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
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
            // eventlisteners::place_ships(x, y, X_DELTA, Y_DELTA, &mut self.player);
            // eventlisteners::placeships
            eventlisteners::place_ships(x, y, &mut self.player);
            eventlisteners::place_ships(x, y, &mut self.enemy);
        }
    
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
