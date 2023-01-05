use ruscii::spatial::{Vec2};
use crate::constant::BORDER_SIZE;
use crate::enemy::Enemy;
use crate::ship::Ship;


pub struct GameState {
    pub dimension: Vec2,
    pub ship: Ship,
    pub enemies: Vec<Enemy>,
    pub lives: usize,
    pub score: usize,
}

impl GameState {
    pub fn new(dimension: Vec2) -> GameState {
        let ship = Ship::new(Vec2::xy(dimension.x / 2, dimension.y - 2));
        let mut enemies = Vec::new();
        for x in 5..dimension.x - 5 {
            if x % 2 != 0 {
                enemies.push(
                    Enemy::new(
                        Vec2::xy(x, BORDER_SIZE),
                        Vec2::y(1),
                        20,
                    )
                );
            }
        }
        GameState {
            dimension,
            ship,
            enemies,
            lives: 3,
            score: 0,
        }
    }

    pub fn move_ship_x(&mut self, dx: i32) {
        self.ship.move_x(dx, self.dimension.x);
    }

    pub fn move_ship_y(&mut self, dy: i32) {
        self.ship.move_y(dy, self.dimension.y);
    }

    pub fn update(&mut self, frame: usize) {
        self.ship.update(frame);
        self.enemies.iter_mut().for_each(|enemy| enemy.update(frame));
    }
}
