use ruscii::spatial::{Vec2};

pub struct GameState {
    pub dimension: Vec2,
    pub spaceship_pos: Vec2,
    pub spaceship_shots: Vec<Vec2>,
    pub last_shot_frame: usize,
    pub lives: usize,
    pub score: usize,
    pub auto_shoot: bool,
}

impl GameState {
    pub fn new(dimension: Vec2) -> GameState {
        GameState {
            dimension,
            spaceship_pos: Vec2::xy(dimension.x / 2, dimension.y - 2),
            spaceship_shots: Vec::new(),
            last_shot_frame: 0,
            lives: 3,
            score: 0,
            auto_shoot: false,
        }
    }

    pub fn move_ship_x(&mut self, dx: i32) {
        let new_x = self.spaceship_pos.x + dx;
        if new_x <= 0 || new_x >= self.dimension.x {
            return;
        }
        self.spaceship_pos.x = new_x;
    }

    pub fn move_ship_y(&mut self, dy: i32) {
        let new_y = self.spaceship_pos.y + dy;
        if new_y <= 0 || new_y >= self.dimension.y {
            return;
        }
        self.spaceship_pos.y = new_y;
    }

    pub fn shoot(&mut self, shot_frame: usize) {
        if self.last_shot_frame + 15 < shot_frame {
            self.spaceship_shots.push(self.spaceship_pos);
            self.last_shot_frame = shot_frame;
        }
    }

    pub fn update(&mut self, frame: usize) {
        if self.auto_shoot {
            self.shoot(frame);
        }
        self.spaceship_shots.retain(|shot| {
            if shot.y == 1 { false } else { true }
        });
        self.spaceship_shots.iter_mut().for_each(|shot| shot.y -= 1);
    }
}
