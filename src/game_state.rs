use rand::{Rng, thread_rng};
use ruscii::spatial::{Vec2};
use crate::constant::{BORDER_SIZE, ENEMY_SPAWN_PROBABILITY};
use crate::enemy::Enemy;
use crate::ship::Ship;
use crate::shot::Shot;

pub struct GameState {
    pub dimension: Vec2,
    pub ship: Ship,
    pub enemies: Vec<Enemy>,
    pub enemy_shots: Vec<Shot>,
    pub lives: usize,
    pub score: usize,
    pub last_spawn: usize,
    pub spawn_interval: usize,
}

impl GameState {
    pub fn new(dimension: Vec2) -> GameState {
        let ship = Ship::new(Vec2::xy(dimension.x / 2, dimension.y - 2));
        GameState {
            dimension,
            ship,
            enemies: Vec::new(),
            enemy_shots: Vec::new(),
            lives: 3,
            score: 0,
            last_spawn: 0,
            spawn_interval: 20,
        }
    }

    pub fn move_ship_x(&mut self, dx: i32) {
        self.ship.move_x(dx, self.dimension.x);
    }

    pub fn move_ship_y(&mut self, dy: i32) {
        self.ship.move_y(dy, self.dimension.y);
    }

    fn spawn_enemy(&mut self, frame: usize) {
        if frame + self.last_spawn > self.spawn_interval {
            let mut rng = thread_rng();
            if rng.gen_bool(ENEMY_SPAWN_PROBABILITY) {
                let x_pos = rng.gen_range(5..self.dimension.x - 5);
                self.enemies.push(
                    Enemy::new(
                        Vec2::xy(x_pos, BORDER_SIZE),
                        Vec2::y(1),
                        20,
                    )
                );
            }
        }
    }

    pub fn update(&mut self, frame: usize) {
        self.ship.update(frame);
        self.enemies.iter_mut().for_each(|enemy| enemy.update(frame, &mut self.enemy_shots));
        self.enemy_shots.retain(|shot| {
            shot.pos.y < self.dimension.y - BORDER_SIZE
        });
        self.enemy_shots.iter_mut().for_each(|shot| {
            shot.update();
        });
        self.spawn_enemy(frame);

        let mut partial_score = 0;
        let enemeies = &mut self.enemies;
        self.ship.shots.retain(|shot| {
            if shot.y == 1 { return false; }
            let pre_len = enemeies.len();
            enemeies.retain(|enemy| &enemy.pos != shot);
            let destroyed = enemeies.len() != pre_len;
            if destroyed {
                partial_score += 5;
            }
            !destroyed
        });
        self.score += partial_score;
    }
}
