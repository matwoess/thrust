use ruscii::drawing::Pencil;
use ruscii::spatial::Vec2;
use ruscii::terminal::Color;

const SHOT_SPEED: i32 = 2;

pub struct Enemy {
    pub pos: Vec2,
    pub shots: Vec<Vec2>,
    pub movement: Vec2,
    pub last_movement: usize,
    pub move_interval: usize,
    pub last_shot_frame: usize,
    pub shot_interval: usize,
}

impl Enemy {
    pub fn new(initial_position: Vec2, movement: Vec2, shot_interval: usize) -> Self {
        Self {
            pos: initial_position,
            shots: Vec::new(),
            movement,
            move_interval: 10,
            last_movement: 0,
            shot_interval,
            last_shot_frame: 0,
        }
    }

    pub fn shoot(&mut self, shot_frame: usize) {
        if self.last_shot_frame + self.shot_interval < shot_frame {
            self.shots.push(self.pos);
            self.last_shot_frame = shot_frame;
        }
    }

    pub fn move_self(&mut self, move_frame: usize) {
        if self.last_movement + self.move_interval < move_frame {
            self.pos += self.movement;
            self.shots.iter_mut().for_each(|shot| shot.y += SHOT_SPEED);
            self.last_movement = move_frame;
        }
    }

    pub fn update(&mut self, frame: usize) {
        self.shoot(frame);
        self.move_self(frame);
    }

    pub fn draw(&self, pencil: &mut Pencil) {
        pencil.set_foreground(Color::Red);
        for shot in &self.shots {
            pencil.draw_char('|', *shot);
        }
        pencil.set_foreground(Color::Green);
        pencil.draw_char('M', self.pos);
    }
}