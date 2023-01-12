use ruscii::drawing::Pencil;
use ruscii::spatial::Vec2;
use ruscii::terminal::Color;

pub struct Shot {
    pub pos: Vec2,
    pub movement: Vec2,
    pub color: Color,
    pub character: char,
}

impl Shot {
    pub fn new(pos: Vec2, movement: Vec2, color: Color, character: char) -> Self {
        Self { pos, movement, color, character }
    }

    pub fn update(&mut self) {
        self.pos += self.movement;
    }

    pub fn draw(&self, pencil: &mut Pencil) {
        pencil.set_foreground(self.color);
        pencil.draw_char(self.character, self.pos);
    }
}