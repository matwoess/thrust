use rand::distributions::{Distribution, Standard};
use rand::Rng;
use ruscii::drawing::Pencil;
use ruscii::spatial::Vec2;
use ruscii::terminal::Color;
use crate::constant::{CHAR_HEALTH, CHAR_SHIELD, CHAR_UPGRADE_SHIP_BASIC, CHAR_UPGRADE_SHIP_DIAGONAL, CHAR_UPGRADE_SHIP_STRONG};
use crate::ship::ShipType;

pub enum GoodieType {
    RepairKit(usize),
    ShieldBoost(usize),
    ShipUpgrade(ShipType),
}

impl Distribution<GoodieType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> GoodieType {
        match rng.gen_range(0..=5) {
            0 | 1 => GoodieType::RepairKit(5),
            2 | 3 | 4 => GoodieType::ShieldBoost(10),
            _ => GoodieType::ShipUpgrade(rand::random()),
        }
    }
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
        let color = match self.goodie_type {
            GoodieType::RepairKit(_) => Color::Red,
            GoodieType::ShieldBoost(_) => Color::Yellow,
            GoodieType::ShipUpgrade(_) => Color::Cyan,
        };
        pencil.set_foreground(color);
        let char_representation = match &self.goodie_type {
            GoodieType::RepairKit(_) => CHAR_HEALTH,
            GoodieType::ShieldBoost(_) => CHAR_SHIELD,
            GoodieType::ShipUpgrade(ship_type) => match ship_type {
                ShipType::Basic => CHAR_UPGRADE_SHIP_BASIC,
                ShipType::DiagonalShot => CHAR_UPGRADE_SHIP_DIAGONAL,
                ShipType::StrongShot => CHAR_UPGRADE_SHIP_STRONG,
            },
        };
        pencil.draw_char(char_representation, self.pos);
    }
}