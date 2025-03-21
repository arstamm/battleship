use ggez::{Context, ContextBuilder, GameResult};
use ggez::event::{self, EventHandler};
// use ggez::graphics::{self, Canvas, Color, DrawParam, Text, TextFragment, Rect};
use ggez::input::mouse::MouseButton;
use ggez::input::keyboard::{KeyInput, KeyCode};
use crate::gameplay::{GridApp, FocusedField};
mod interactive;
mod gameplay;
mod display;

impl EventHandler for GridApp {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.update(ctx)
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keyinput: KeyInput,
        _repeat: bool,
    ) -> GameResult {
        if self.input_mode {
            match keyinput.keycode {
                Some(KeyCode::Back) => {
                    match self.focused_field {
                        FocusedField::Row => {
                            self.row_input.pop();
                        }
                        FocusedField::Column => {
                            self.col_input.pop();
                        }
                        FocusedField::Value => {
                            self.value_input.pop();
                        }
                    }
                    return Ok(());
                }
                Some(KeyCode::Return) => {
                    self.apply_input();
                    return Ok(());
                }
                Some(KeyCode::Tab) => {
                    self.focused_field = match self.focused_field {
                        FocusedField::Row => FocusedField::Column,
                        FocusedField::Column => FocusedField::Value,
                        FocusedField::Value => FocusedField::Row,
                    };
                    return Ok(());
                }
                Some(key) => {
                    let char = match key {
                        KeyCode::A => 'A',
                        KeyCode::B => 'B',
                        KeyCode::C => 'C',
                        KeyCode::D => 'D',
                        KeyCode::E => 'E',
                        KeyCode::F => 'F',
                        KeyCode::G => 'G',
                        KeyCode::H => 'H',
                        KeyCode::I => 'I',
                        KeyCode::J => 'J',
                        KeyCode::K => 'K',
                        KeyCode::L => 'L',
                        KeyCode::M => 'M',
                        KeyCode::N => 'N',
                        KeyCode::O => 'O',
                        KeyCode::P => 'P',
                        KeyCode::Q => 'Q',
                        KeyCode::R => 'R',
                        KeyCode::S => 'S',
                        KeyCode::T => 'T',
                        KeyCode::U => 'U',
                        KeyCode::V => 'V',
                        KeyCode::W => 'W',
                        KeyCode::X => 'X',
                        KeyCode::Y => 'Y',
                        KeyCode::Z => 'Z',
                        KeyCode::Key0 => '0',
                        KeyCode::Key1 => '1',
                        KeyCode::Key2 => '2',
                        KeyCode::Key3 => '3',
                        KeyCode::Key4 => '4',
                        KeyCode::Key5 => '5',
                        KeyCode::Key6 => '6',
                        KeyCode::Key7 => '7',
                        KeyCode::Key8 => '8',
                        KeyCode::Key9 => '9',
                        _ => return Ok(()),
                    };

                    match self.focused_field {
                        FocusedField::Row => self.row_input.push(char),
                        FocusedField::Column => self.col_input.push(char),
                        FocusedField::Value => self.value_input.push(char),
                    }
                }
                None => return Ok(()),
            };
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        self.render(ctx) // Call render instead of draw
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) -> GameResult {
        if button == MouseButton::Left && !self.input_mode {
            let (screen_w, screen_h) = _ctx.gfx.drawable_size();
            let button_width = 100.0;
            let button_height = 30.0;
            let button_x = (screen_w - button_width) / 2.0;
            let button_y = screen_h - button_height - 20.0;

            if x >= button_x && x <= button_x + button_width && y >= button_y && y <= button_y + button_height {
                println!("Button clicked!");
                self.open_input_dialog();
            }
        }
        Ok(())
    }
}

fn main() -> GameResult {
    println!("Hello World");
    let (ctx, event_loop) = ContextBuilder::new("grid_app", "Author")
        .build()
        .expect("Failed to build ggez context");

    let app = GridApp::new();
    event::run(ctx, event_loop, app)
}