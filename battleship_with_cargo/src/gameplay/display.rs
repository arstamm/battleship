use ggez::{Context, GameResult};
use ggez::event::{EventHandler, MouseButton};
use ggez::graphics;
use ggez::input::keyboard::{KeyInput, KeyCode};

use crate::gameplay::gameplay::BattleshipGame;
use crate::gameplay::eventlisteners;
use crate::gameplay::constants;

impl EventHandler for BattleshipGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        eventlisteners::check_win(&mut self.player);
        eventlisteners::check_win(&mut self.enemy);   
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, constants::BACKGROUND_COLOR);
        
        // Display Grids
        BattleshipGame::display_grid(&self.player, &mut canvas, ctx)?;
        BattleshipGame::display_grid(&self.enemy, &mut canvas, ctx)?;

        // Display Banners
        BattleshipGame::display_banner(&self.text_display_box, ctx, &mut canvas)?;
        BattleshipGame::display_banner(&self.p1_button, ctx, &mut canvas)?;
        BattleshipGame::display_banner(&self.p2_button, ctx, &mut canvas)?;


        canvas.finish(ctx)?;
        Ok(())
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) -> GameResult<()> {
        if button == MouseButton::Left {
            eventlisteners::place_ships(x, y, &mut self.player);
            eventlisteners::place_ships(x, y, &mut self.enemy);
            eventlisteners::click_action(x, y, &mut self.p1_button, &mut self.player);
            eventlisteners::click_action(x, y, &mut self.p2_button, &mut self.enemy);
            eventlisteners::check_guess(x, y, &mut self.player);
            eventlisteners::check_guess(x, y, &mut self.enemy);
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
