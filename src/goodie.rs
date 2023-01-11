use rand::distributions::{Distribution, Standard};
use rand::Rng;
use ruscii::drawing::Pencil;
use ruscii::spatial::Vec2;
use ruscii::terminal::Color;
use crate::constant::{CHAR_UPGRADE_SHIP_BASIC, CHAR_UPGRADE_SHIP_DIAGONAL, CHAR_UPGRADE_SHIP_STRONG, HEALTH_CHAR, SHIELD_CHAR};
use crate::ship::ShipType;

pub enum GoodieType {
    RepairKit(usize),
    ShieldBoost(usize),
    ShipUpgrade(ShipType),
}

impl Distribution<GoodieType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> GoodieType {
        match rng.gen_range(0..=2) {
            0 => GoodieType::RepairKit(5),
            1 => GoodieType::ShieldBoost(5),
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
        pencil.set_foreground(Color::Magenta);
        let char_representation = match &self.goodie_type {
            GoodieType::RepairKit(_) => HEALTH_CHAR,
            GoodieType::ShieldBoost(_) => SHIELD_CHAR,
            GoodieType::ShipUpgrade(ship_type) => match ship_type {
                ShipType::Basic => CHAR_UPGRADE_SHIP_BASIC,
                ShipType::DiagonalShot => CHAR_UPGRADE_SHIP_DIAGONAL,
                ShipType::StrongShot => CHAR_UPGRADE_SHIP_STRONG,
            },
        };
        pencil.draw_char(char_representation, self.pos);
    }
}