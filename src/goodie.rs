use ruscii::drawing::Pencil;
use ruscii::spatial::Vec2;
use ruscii::terminal::Color;
use crate::ship::ShipType;

pub enum GoodieType {
    RepairKit(usize),
    ShieldBoost(usize),
    ShipUpgrade(ShipType),
}

pub struct Goodie {
    pub pos: Vec2,
    pub goodie_type: GoodieType,
}

impl Goodie {
    pub fn new(pos: Vec2, goodie_type: GoodieType) -> Self {
        Self { pos, goodie_type }
    }

    pub fn update(&mut self) {
        self.pos.y += 1;
    }

    pub fn draw(&self, pencil: &mut Pencil) {
        pencil.set_foreground(Color::Magenta);
        let char_representation = match &self.goodie_type {
            GoodieType::RepairKit(_) => '♥',
            GoodieType::ShieldBoost(_) => '⛨',
            GoodieType::ShipUpgrade(ship_type) => match ship_type {
                ShipType::Basic => '^',
                ShipType::DiagonalShot => 'Y',
                ShipType::StrongShot => 'T',
            },
        };
        pencil.draw_char(char_representation, self.pos);
    }
}