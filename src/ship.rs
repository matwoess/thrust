use rand::distributions::{Distribution, Standard};
use rand::Rng;
use ruscii::drawing::Pencil;
use ruscii::spatial::Vec2;
use ruscii::terminal::{Color, Style};

pub enum ShipType {
  Basic, DiagonalShot, StrongShot
}

impl Distribution<ShipType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> ShipType {
        match rng.gen_range(0..=2) {
            0 => ShipType::Basic,
            1 => ShipType::DiagonalShot,
            _ => ShipType::StrongShot,
        }
    }
}

pub struct Ship {
    pub pos: Vec2,
    pub ship_type: ShipType,
    pub shots: Vec<Vec2>,
    pub auto_shoot: bool,
    pub shot_interval: usize,
    pub last_shot_frame: usize,
}

impl Ship {
    pub(crate) fn change_ship_type(&mut self, new_ship_type: &ShipType) {
        self.ship_type = match new_ship_type {
            ShipType::Basic => ShipType::Basic,
            ShipType::DiagonalShot => ShipType::DiagonalShot,
            ShipType::StrongShot => ShipType::StrongShot,
        };
    }
}

impl Ship {
    pub fn new(initial_position: Vec2) -> Self {
        Self {
            pos: initial_position,
            ship_type: ShipType::Basic,
            shots: Vec::new(),
            last_shot_frame: 0,
            auto_shoot: false,
            shot_interval: 15,
        }
    }

    pub fn move_x(&mut self, dx: i32, max_x: i32) {
        let new_x = self.pos.x + dx;
        if new_x <= 0 || new_x >= max_x {
            return;
        }
        self.pos.x = new_x;
    }
    pub fn move_y(&mut self, dy: i32, max_y: i32) {
        let new_y = self.pos.y + dy;
        if new_y <= 0 || new_y >= max_y {
            return;
        }
        self.pos.y = new_y;
    }

    pub fn shoot(&mut self, shot_frame: usize) {
        if self.last_shot_frame + self.shot_interval < shot_frame {
            self.shots.push(self.pos);
            self.shots.push(self.pos + Vec2::x(1));
            self.last_shot_frame = shot_frame;
        }
    }

    pub fn update(&mut self, frame: usize) {
        if self.auto_shoot {
            self.shoot(frame);
        }
        self.shots.iter_mut().for_each(|shot| shot.y -= 1);
    }

    pub fn is_hit_by(&self, object: &Vec2) -> bool {
        if object.y == self.pos.y {
            if object.x == self.pos.x || object.x == self.pos.x + 1 {
                return true;
            }
        }
        false
    }

    pub fn draw(&self, pencil: &mut Pencil) {
        pencil.set_foreground(Color::Cyan);
        pencil.draw_char('/', self.pos);
        pencil.draw_char('\\', self.pos + Vec2::x(1));
        pencil.set_foreground(Color::Yellow);
        pencil.set_style(Style::Bold);
        for shot in &self.shots {
            pencil.draw_char('|', *shot);
        }
    }
}