// Constants

// Imports
use ggez::conf::{WindowMode, FullscreenType};
use ggez::graphics::Color;

// Colors

// 232, 233, 243 Ghost White
const GHOST_WHITE: Color = Color::new(0.9098039, 0.9137255, 0.9529412, 1.0);

// 255, 27, 28 Red
const RED: Color = Color::new(1.0000000, 0.1058824, 0.1098039, 1.0);

// 166, 166, 168 French Gray
const FRENCH_GRAY: Color = Color::new(0.6509804, 0.6509804, 0.6588235, 1.0);

// 206, 206, 206 Silver
// const SILVER: Color = Color::new(0.8078431, 0.8078431, 0.8078431, 1.0);

// 39, 38, 53 Raisin Black
const RAISIN_BLACK: Color = Color::new(0.1529412, 0.1490196, 0.2078431, 1.0);

// 10, 76, 126 Marine Blue
const MARINE_BLUE: Color = Color::new(0.0392157, 0.2980392, 0.4941176, 1.0);

// 19, 159, 236 Blue
const BLUE: Color = Color::new(0.0745098, 0.6235294, 0.9254902, 1.0);

// 158, 228, 147 Kelly Green
// const KELLY_GREEN: Color = Color::new(0.6196078, 0.8941176, 0.5764706, 1.0);

// 91, 140, 90 Sea Green
const SEA_GREEN: Color = Color::new(0.3568627, 0.5490196, 0.3529412, 1.0);


// Title Bar
pub const TITLE_BAR: &str = "Battleship";

// Window Set Up
const SCREEN_SCALE: f32 = 1.3;
pub const BACKGROUND_COLOR: Color = RAISIN_BLACK;

pub const GAME_WINDOW: WindowMode = WindowMode {
    width: 2240.0 * SCREEN_SCALE, //2240
    height: 1260.0 * SCREEN_SCALE, //1260
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

// Grid
pub const GRID_CELL_COLOR: Color = BLUE;
pub const GRID_LINE_COLOR: Color = Color::BLACK;
pub const SHIP_COLOR: Color = SEA_GREEN;
pub const HIT_COLOR: Color = RED;
pub const HOVER_COLOR: Color = MARINE_BLUE;
pub const MISS_COLOR: Color = GHOST_WHITE;
pub const X_DELTA: f32 = 80.0 * SCREEN_SCALE;
pub const Y_DELTA: f32 = 80.0 * SCREEN_SCALE;
pub const X_DELTA_ENEMY: f32 = 980.0 * SCREEN_SCALE;
pub const GRID_SIZE: usize = 10; // This needs to be usize specifically.
pub const CELL_SIZE: f32 = 80.0 * SCREEN_SCALE;
pub const SHIP_SIZES: [usize; 5] = [5, 4, 3, 3, 2];
// pub const MAX_HIT_POINTS = 

// Text
pub const TEXT_MARGIN: f32 = 30.0 * SCREEN_SCALE;
pub const FONT_SIZE: f32 = 60.0 * SCREEN_SCALE;
pub const FONT_COLOR: Color = RAISIN_BLACK;

// Banner
pub const BANNER_BACKGROUND_COLOR: Color = FRENCH_GRAY;
pub const BANNER_X_POS: f32 = X_DELTA;
pub const BANNER_Y_POS: f32 = 960.0 * SCREEN_SCALE;
pub const BANNER_WIDTH: f32 = 1700.0 * SCREEN_SCALE;
pub const BANNER_HEIGHT: f32 = 130.0 * SCREEN_SCALE;

// Buttons
pub const BUTTON_BACKGROUND_COLOR: Color = FRENCH_GRAY;
pub const BUTTON_X_POS: f32 = 1860.0 * SCREEN_SCALE;
pub const BUTTON_Y_POS: f32 = 240.0 * SCREEN_SCALE;
// pub const BUTTON_ENEMY_Y_POS: f32 = 580.0 * SCREEN_SCALE;
pub const BUTTON_WIDTH: f32 = 300.0 * SCREEN_SCALE;
pub const BUTTON_HEIGHT: f32 = 240.0 * SCREEN_SCALE;

// Message string Fragments
