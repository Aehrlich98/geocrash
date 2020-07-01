use ggez::graphics;

pub const LEFT: i8 = 0;
pub const RIGHT: i8 = 1;
pub const UP: i8 = 2;
pub const DOWN: i8 = 3;
pub const PLAYER1_ID: i8 = 4;
pub const PLAYER2_ID: i8 = 5;
pub const DEFAULT_ID: i8 = 6;
pub const BOUND_ID: i8 = 7;
//the numbers 50 to 100 are used for individual game object ids
pub const GAME_SIZE: i32 = 600;
pub const PLAYER1_COLOR: graphics::Color = graphics::Color::new(0.8, 0.0, 0.0, 0.7);
pub const PLAYER2_COLOR: graphics::Color = graphics::Color::new(0.0, 0.8, 0.0, 0.7);
pub const DEFAULT_COLOR: graphics::Color = graphics::Color::new(0.0, 1.0, 1.0, 0.90);