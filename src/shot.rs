use ruscii::drawing::Pencil;
use ruscii::spatial::Vec2;
use ruscii::terminal::Color;

pub struct Shot {
    pub pos: Vec2,
}

impl Shot {
    pub fn new(pos: Vec2) -> Self {
        Self {pos}
    }
    pub fn update(&mut self) {
        self.pos.y += 1;
    }
    pub fn draw(&self, pencil: &mut Pencil) {
        pencil.set_foreground(Color::Red);
        pencil.draw_char('|', self.pos);
    }
}