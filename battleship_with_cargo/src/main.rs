use ggez::{Context, ContextBuilder, GameResult};
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Canvas, Color, DrawParam, Text, TextFragment, Rect};
use ggez::input::mouse::MouseButton;
use ggez::graphics::Drawable;
use ggez::input::keyboard::KeyInput;
use ggez::input::keyboard::KeyCode;
use std::io::{self, Write};
mod enemy;

enum FocusedField {
    Row,
    Column,
    Value,
}

struct GridApp {
    grid1: [[i32; 10]; 10],
    grid2: [[i32; 10]; 10],
    input_mode: bool,
    row_input: String,
    col_input: String,
    value_input: String,
    focused_field: FocusedField, // Track which field is focused
}

impl GridApp {
    fn new() -> Self {
        GridApp {
            grid1: [[0; 10]; 10],
            grid2: [[0; 10]; 10],
            input_mode: false,
            row_input: String::new(),
            col_input: String::new(),
            value_input: String::new(),
            focused_field: FocusedField::Row,
        }
    }

    fn open_input_dialog(&mut self) {
        self.input_mode = true;
    }

    fn close_input_dialog(&mut self) {
        self.input_mode = false;
        self.row_input.clear();
        self.col_input.clear();
        self.value_input.clear();
    }

    fn apply_input(&mut self) {
        let row = match self.row_input.trim().to_uppercase().as_str() {
            "A" => 0, "B" => 1, "C" => 2, "D" => 3, "E" => 4,
            "F" => 5, "G" => 6, "H" => 7, "I" => 8, "J" => 9,
            _ => {
                println!("Invalid row: {}", self.row_input);
                return;
            }
        };
    
        let col: usize = match self.col_input.trim().parse() {
            Ok(n) if (1..=10).contains(&n) => (n - 1) as usize,
            _ => {
                println!("Invalid column: {}", self.col_input);
                return;
            }
        };
    
        let value: i32 = match self.value_input.trim().parse() {
            Ok(v) => v,
            _ => {
                println!("Invalid value: {}", self.value_input);
                return;
            }
        };
    
        // Update the grid1 at the specified row and column
        self.grid1[col][row] = value;
        println!("Grid 1: Set value {} at row {}, column {}", value, row, col);
    
        self.close_input_dialog();
    }
}

impl EventHandler for GridApp {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keyinput: KeyInput,  // We get KeyInput here
        _repeat: bool,
    ) -> GameResult {
        if self.input_mode {
            let input_str = match keyinput.keycode {  // Use keycode from KeyInput
                Some(KeyCode::Back) => {  // Handle Backspace key
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
                Some(KeyCode::Return) => {  // Handle Enter key
                    self.apply_input();
                    return Ok(());
                }
                Some(KeyCode::Tab) => {  // Handle Tab key to switch focus
                    self.focused_field = match self.focused_field {
                        FocusedField::Row => FocusedField::Column,
                        FocusedField::Column => FocusedField::Value,
                        FocusedField::Value => FocusedField::Row,
                    };
                    return Ok(());
                }
                Some(key) => {  // Handle all other keys
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
                        _ => return Ok(()),  // Return early if the key is not handled
                    };
    
                    match self.focused_field {
                        FocusedField::Row => self.row_input.push(char),
                        FocusedField::Column => self.col_input.push(char),
                        FocusedField::Value => self.value_input.push(char),
                    }
                }
                None => return Ok(()),  // Return early if no keycode is present
            };
        }
    
        Ok(())  // Return Ok() at the end to satisfy the return type
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, Color::from_rgb(255, 255, 255));
    
        // Draw Grid 1
        let title1 = Text::new(TextFragment::new("Grid 1").scale(20.0).color(Color::from_rgb(0, 0, 0)));
        canvas.draw(&title1, DrawParam::default().dest([20.0, 0.0]));
    
        let mut y_offset = 30.0;
        for row in &self.grid1 {
            let row_text = row.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(" ");
            let text = Text::new(TextFragment::new(row_text).scale(18.0).color(Color::from_rgb(0, 0, 0)));
            canvas.draw(&text, DrawParam::default().dest([20.0, y_offset]));
            y_offset += 25.0;
        }
    
        // Draw Grid 2
        let title2 = Text::new(TextFragment::new("Grid 2").scale(20.0).color(Color::from_rgb(0, 0, 0)));
        canvas.draw(&title2, DrawParam::default().dest([300.0, 0.0]));
    
        let mut y_offset = 30.0;
        for row in &self.grid2 {
            let row_text = row.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(" ");
            let text = Text::new(TextFragment::new(row_text).scale(18.0).color(Color::from_rgb(0, 0, 0)));
            canvas.draw(&text, DrawParam::default().dest([300.0, y_offset]));
            y_offset += 25.0;
        }
    
        // Draw button
        if !self.input_mode {
            let button_text = Text::new(TextFragment::new("Click Me").scale(18.0).color(Color::from_rgb(0, 0, 0)));
            let (screen_w, screen_h) = ctx.gfx.drawable_size();
            let button_width = 100.0;
            let button_height = 30.0;
            let button_x = (screen_w - button_width) / 2.0;
            let button_y = screen_h - button_height - 20.0;
    
            let button_rect = Rect::new(button_x, button_y, button_width, button_height);
            canvas.draw(&graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                button_rect,
                Color::from_rgb(200, 200, 200),
            )?, DrawParam::default());
    
            canvas.draw(&button_text, DrawParam::default().dest([button_x + 10.0, button_y + 5.0]));
        }
    
        // Draw input dialog at bottom center
        if self.input_mode {
            let input_text = format!("Row: {}\nColumn: {}\nValue: {}", self.row_input, self.col_input, self.value_input);
            let input_box = Text::new(TextFragment::new(input_text).scale(18.0).color(Color::from_rgb(0, 0, 0)));
    
            // Get the dimensions of the input box
            if let Some(input_box_rect) = input_box.dimensions(ctx) {
                let input_box_width = input_box_rect.w;
                let input_box_height = input_box_rect.h;
    
                // Calculate the x and y coordinates to position the dialog at the bottom center
                let (screen_w, screen_h) = ctx.gfx.drawable_size();
                let x = (screen_w - input_box_width) / 2.0;
                let y = screen_h - input_box_height - 20.0;  // Position just above the bottom
    
                canvas.draw(&input_box, DrawParam::default().dest([x, y]));
            }
        }

        if self.input_mode {
            let row_label = Text::new(TextFragment::new("Row: ").scale(18.0).color(Color::from_rgb(0, 0, 0)));
            let col_label = Text::new(TextFragment::new("Column: ").scale(18.0).color(Color::from_rgb(0, 0, 0)));
            let value_label = Text::new(TextFragment::new("Value: ").scale(18.0).color(Color::from_rgb(0, 0, 0)));
    
            // Get the dimensions of the input box
            let row_input_text = Text::new(TextFragment::new(&self.row_input).scale(18.0).color(Color::from_rgb(0, 0, 0)));
            let col_input_text = Text::new(TextFragment::new(&self.col_input).scale(18.0).color(Color::from_rgb(0, 0, 0)));
            let value_input_text = Text::new(TextFragment::new(&self.value_input).scale(18.0).color(Color::from_rgb(0, 0, 0)));
    
            // Calculate the x and y coordinates to position the dialog at the bottom center
            let (screen_w, screen_h) = ctx.gfx.drawable_size();
            let y = screen_h - 150.0; // Position just above the bottom
    
            // Draw input fields
            canvas.draw(&row_label, DrawParam::default().dest([screen_w / 2.0 - 100.0, y]));
            canvas.draw(&row_input_text, DrawParam::default().dest([screen_w / 2.0, y]));
            canvas.draw(&col_label, DrawParam::default().dest([screen_w / 2.0 - 100.0, y + 30.0]));
            canvas.draw(&col_input_text, DrawParam::default().dest([screen_w / 2.0, y + 30.0]));
            canvas.draw(&value_label, DrawParam::default().dest([screen_w / 2.0 - 100.0, y + 60.0]));
            canvas.draw(&value_input_text, DrawParam::default().dest([screen_w / 2.0, y + 60.0]));
        }
    
        canvas.finish(ctx)
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
    let (ctx, event_loop) = ContextBuilder::new("grid_app", "Author")
        .build()
        .expect("Failed to build ggez context");

    let app = GridApp::new();
    event::run(ctx, event_loop, app)
}
