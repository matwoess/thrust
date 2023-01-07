use std::cmp::{max};
use rand::{Rng, thread_rng};
use ruscii::spatial::{Vec2};
use crate::constant::{BORDER_SIZE, DMG_COLLISION, DMG_ENEMY_REACHED_GROUND, DMG_SHOT_HIT, FPS_LIMIT, MIN_SPAWN_INTERVAL, SPAWN_INTERVAL_DECREASE, SPEEDUP_AFTER_X_FRAMES};
use crate::enemy::Enemy;
use crate::ship::Ship;
use crate::shot::Shot;

pub struct GameState {
    pub dimension: Vec2,
    pub ship: Ship,
    pub enemies: Vec<Enemy>,
    pub enemy_shots: Vec<Shot>,
    pub health: isize,
    pub score: usize,
    pub last_spawn: usize,
    pub spawn_interval: usize,
    pub last_spawn_speedup: usize,
}

impl GameState {
    pub fn new(dimension: Vec2) -> GameState {
        let ship = Ship::new(Vec2::xy(dimension.x / 2, dimension.y - 2));
        GameState {
            dimension,
            ship,
            enemies: Vec::new(),
            enemy_shots: Vec::new(),
            health: 100,
            score: 0,
            last_spawn: 0,
            spawn_interval: (2 * FPS_LIMIT) as usize,
            last_spawn_speedup: 0,
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
        self.update_enemies(frame);
        self.update_enemy_shots();
        self.spawn_enemy(frame);
        self.update_ship_shots();
        self.update_game_speed(frame);
    }

    fn update_enemies(&mut self, frame: usize) {
        self.enemies.iter_mut().for_each(|enemy| enemy.update(frame, &mut self.enemy_shots));
        self.enemies.retain(|enemy| {
            if self.ship.is_hit_by(&enemy.pos) {
                self.health -= DMG_COLLISION;
                return false;
            }
            if enemy.pos.y > self.dimension.y - BORDER_SIZE {
                self.health -= DMG_ENEMY_REACHED_GROUND;
                return false;
            }
            true
        });
    }

    fn update_enemy_shots(&mut self) {
        self.enemy_shots.retain(|shot| {
            if self.ship.is_hit_by(&shot.pos) {
                self.health -= DMG_SHOT_HIT;
                return false;
            }
            shot.pos.y < self.dimension.y - BORDER_SIZE
        });
        self.enemy_shots.iter_mut().for_each(|shot| {
            shot.update();
        });
    }

    fn spawn_enemy(&mut self, frame: usize) {
        if self.last_spawn + self.spawn_interval < frame {
            self.last_spawn = frame;
            let x_pos = thread_rng().gen_range(5..self.dimension.x - 5);
            self.enemies.push(
                Enemy::new(
                    Vec2::xy(x_pos, BORDER_SIZE),
                    Vec2::y(1),
                    FPS_LIMIT as usize,
                )
            );
        }
    }

    fn update_ship_shots(&mut self) {
        let mut partial_score = 0;
        let enemies = &mut self.enemies;
        self.ship.shots.retain(|shot| {
            if shot.y == 1 { return false; }
            let pre_len = enemies.len();
            enemies.retain(|enemy| &enemy.pos != shot);
            let destroyed = enemies.len() != pre_len;
            if destroyed {
                partial_score += 5;
            }
            !destroyed
        });
        self.score += partial_score;
    }

    fn update_game_speed(&mut self, frame: usize) {
        if self.last_spawn_speedup + SPEEDUP_AFTER_X_FRAMES < frame {
            self.spawn_interval = max(MIN_SPAWN_INTERVAL, self.spawn_interval - SPAWN_INTERVAL_DECREASE);
            self.last_spawn_speedup = frame;
        }
    }
}
