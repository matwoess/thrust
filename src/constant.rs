pub const FPS_LIMIT: u32 = 20;
pub const MOVE_SPEED_X: i32 = 2;
pub const MOVE_SPEED_Y: i32 = 1;
pub const GAME_SIZE: (i32, i32) = (60, 32);
pub const BORDER_SIZE: i32 = 1;

pub const ENEMY_SHOT_PROBABILITY: f64 = 0.2;

pub const DMG_ENEMY_REACHED_GROUND: isize = 15;
pub const DMG_COLLISION: isize = 50;
pub const DMG_SHOT_HIT: isize = 5;

pub const SPEEDUP_AFTER_X_FRAMES: usize = (FPS_LIMIT * 10) as usize;
pub const SPAWN_INTERVAL_DECREASE: usize = 10;
pub const MIN_SPAWN_INTERVAL: usize = (FPS_LIMIT / 2) as usize;