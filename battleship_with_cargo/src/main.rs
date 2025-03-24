use ggez::{Context, GameResult};
use ggez::event::{self, EventHandler, MouseButton};
use ggez::graphics::{self, Color, DrawMode, MeshBuilder, Text, PxScale, Mesh};
use ggez::mint::{Point2, Vector2};

mod gameplay;

use gameplay::BattleshipGame;


fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("battleship", "author");
    let (ctx, event_loop) = cb.build()?;
    let game = BattleshipGame::new();
    event::run(ctx, event_loop, game)
}


// #[derive(Clone, Copy, PartialEq)]
// enum CellState {
//     Empty,
//     Ship,
//     Hit,
//     Miss,
// }

// struct BattleshipGame {
//     player_grid: [[CellState; GRID_SIZE]; GRID_SIZE],
//     enemy_grid: [[CellState; GRID_SIZE]; GRID_SIZE],
//     player_turn: bool,
//     placing_ships: bool,
//     current_ship_index: usize,
// }

// impl BattleshipGame {
//     fn new() -> Self {
//         Self {
//             player_grid: [[CellState::Empty; GRID_SIZE]; GRID_SIZE],
//             enemy_grid: [[CellState::Empty; GRID_SIZE]; GRID_SIZE],
//             player_turn: false,
//             placing_ships: true,
//             current_ship_index: 0,
//         }
//     }

//     fn handle_click(&mut self, x: f32, y: f32) {
//         let col = (x / CELL_SIZE) as usize;
//         let row = (y / CELL_SIZE) as usize;
//         if row < GRID_SIZE && col < GRID_SIZE {
//             if self.placing_ships {
//                 if self.current_ship_index < SHIP_SIZES.len() {
//                     if place_ship(&mut self.player_grid, row, col, 1, SHIP_SIZES[self.current_ship_index]) {
//                         self.current_ship_index += 1;
//                         if self.current_ship_index >= SHIP_SIZES.len() {
//                             self.placing_ships = false;
//                             self.player_turn = true;
//                         }
//                     }
//                 }
//             } else if self.player_turn {
//                 match self.enemy_grid[row][col] {
//                     CellState::Ship => self.enemy_grid[row][col] = CellState::Hit,
//                     CellState::Empty => self.enemy_grid[row][col] = CellState::Miss,
//                     _ => return,
//                 }
//                 self.player_turn = false;
//             }
//         }
//     }
// }

// impl EventHandler for BattleshipGame {
//     fn update(&mut self, _ctx: &mut Context) -> GameResult {
//         Ok(())
//     }

//     fn draw(&mut self, ctx: &mut Context) -> GameResult {
//         let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);
//         let mut mesh_builder = MeshBuilder::new();
        
//         for row in 0..GRID_SIZE {
//             for col in 0..GRID_SIZE {
//                 let x = col as f32 * CELL_SIZE;
//                 let y = row as f32 * CELL_SIZE;
//                 let color = match self.player_grid[row][col] {
//                     CellState::Ship => Color::GREEN,
//                     _ => Color::BLUE,
//                 };
                
//                 // Draw cell background
//                 canvas.draw(
//                     &graphics::Quad,
//                     graphics::DrawParam::new()
//                         .dest(Point2 { x, y })
//                         .scale(Vector2 { x: CELL_SIZE, y: CELL_SIZE })
//                         .color(color),
//                 );
                
//                 // Draw grid outline
//                 mesh_builder.line(&[
//                     Point2 { x, y },
//                     Point2 { x: x + CELL_SIZE, y },
//                     Point2 { x: x + CELL_SIZE, y: y + CELL_SIZE },
//                     Point2 { x, y: y + CELL_SIZE },
//                     Point2 { x, y }
//                 ], 1.0, Color::BLACK)?;
//             }
//         }
        
//         let mesh = Mesh::from_data(ctx, mesh_builder.build());
//         canvas.draw(&mesh, graphics::DrawParam::default());
        
//         canvas.finish(ctx)?;
//         Ok(())
//     }

//     fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) -> GameResult<()> {
//         if button == MouseButton::Left {
//             self.handle_click(x, y);
//         }
//         Ok(())
//     }
// }

