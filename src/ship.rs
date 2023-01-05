use ruscii::drawing::Pencil;
use ruscii::spatial::Vec2;
use ruscii::terminal::{Color, Style};

pub struct Ship {
    pub pos: Vec2,
    pub shots: Vec<Vec2>,
    pub last_shot_frame: usize,
    pub auto_shoot: bool,
    pub shot_interval: usize,
}

impl Ship {
    pub fn new(initial_position: Vec2) -> Self {
        Self {
            pos: initial_position,
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
            self.last_shot_frame = shot_frame;
        }
    }

    pub fn update(&mut self, frame: usize) {
        if self.auto_shoot {
            self.shoot(frame);
        }
        self.shots.iter_mut().for_each(|shot| shot.y -= 1);
    }

    pub fn draw(&self, pencil: &mut Pencil) {
        pencil.set_foreground(Color::Cyan);
        pencil.draw_char('A', self.pos);
        pencil.set_foreground(Color::Cyan);
        pencil.set_style(Style::Bold);
        for shot in &self.shots {
            pencil.draw_char('|', *shot);
        }
    }
}