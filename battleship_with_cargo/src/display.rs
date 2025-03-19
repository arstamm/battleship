use ggez::{Context, GameResult};
use ggez::graphics::{self, Canvas, Color, DrawParam, Text, TextFragment};
use crate::gameplay::GridApp;

impl GridApp {
    pub fn draw_grids(&self, ctx: &mut Context, canvas: &mut Canvas) {
        self.draw_grid(ctx, canvas, &self.grid1, "Grid 1", 20.0);
        self.draw_grid(ctx, canvas, &self.grid2, "Grid 2", 300.0);
    }

    fn draw_grid(&self, ctx: &mut Context, canvas: &mut Canvas, grid: &[[usize; 10]; 10], title: &str, x_offset: f32) {
        let title_text = Text::new(TextFragment::new(title).scale(20.0).color(Color::from_rgb(0, 0, 0)));
        canvas.draw(&title_text, DrawParam::default().dest([x_offset, 0.0]));
        
        let mut y_offset = 30.0;
        for row in grid {
            let row_text = row.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(" ");
            let text = Text::new(TextFragment::new(row_text).scale(18.0).color(Color::from_rgb(0, 0, 0)));
            canvas.draw(&text, DrawParam::default().dest([x_offset, y_offset]));
            y_offset += 25.0;
        }
    }
    
    pub fn draw_input_dialog(&self, ctx: &mut Context, canvas: &mut Canvas) {
        if self.input_mode {
            let row_label = Text::new("Row: ");
            let col_label = Text::new("Column: ");
            let value_label = Text::new("Value: ");
            
            let row_input_text = Text::new(&self.row_input);
            let col_input_text = Text::new(&self.col_input);
            let value_input_text = Text::new(&self.value_input);

            let (screen_w, screen_h) = ctx.gfx.drawable_size();
            let y = screen_h - 150.0;

            canvas.draw(&row_label, DrawParam::default().dest([screen_w / 2.0 - 100.0, y]));
            canvas.draw(&row_input_text, DrawParam::default().dest([screen_w / 2.0, y]));
            canvas.draw(&col_label, DrawParam::default().dest([screen_w / 2.0 - 100.0, y + 30.0]));
            canvas.draw(&col_input_text, DrawParam::default().dest([screen_w / 2.0, y + 30.0]));
            canvas.draw(&value_label, DrawParam::default().dest([screen_w / 2.0 - 100.0, y + 60.0]));
            canvas.draw(&value_input_text, DrawParam::default().dest([screen_w / 2.0, y + 60.0]));
        }
    }
    
    pub fn render(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, Color::from_rgb(255, 255, 255));
        self.draw_grids(ctx, &mut canvas);
        self.draw_input_dialog(ctx, &mut canvas);
        canvas.finish(ctx)
    }
}