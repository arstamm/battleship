// Constants

// Imports

use ggez::conf::{WindowMode, FullscreenType}; 

// Strings
pub const TITLE_BAR: &str = "Battleship";

// Window Set Up

const SCREEN_SCALE: f32 = 1.4;

pub const GAME_WINDOW: WindowMode = WindowMode {
    width: 1600.0 * SCREEN_SCALE,
    height: 900.0 * SCREEN_SCALE,
    maximized: false,
    fullscreen_type: FullscreenType::Windowed,
    borderless: false,
    min_width: 1.0,
    max_width: 0.0,
    min_height: 1.0,
    max_height: 0.0,
    resizable: false,
    visible: true,
    transparent: false,
    resize_on_scale_factor_change: false,
    logical_size: None,
};

// Grid and Ship Sizes
pub const GRID_SIZE: usize = 10; // This needs to be usize specifically.
pub const CELL_SIZE: f32 = 80.0;
pub const SHIP_SIZES: [usize; 5] = [5, 4, 3, 3, 2];

// Grid Position
pub const X_DELTA: f32 = 80.0;
pub const Y_DELTA: f32 = 80.0;
pub const X_DELTA_ENEMY: f32 = 980.0;